// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Bringing the gen3 radio up on the claimed card, once, before the serving
//! loop starts: check the register window, take the NIC (which makes the RF
//! id readable), choose the firmware by MAC and RF type, map the control,
//! receive and firmware regions as broker-sized grants, boot to ALIVE and run
//! the post-ALIVE commands. Each step that fails names itself (`Failure`) and
//! prints one line; a failure after the device was told where its memory is
//! stops the device and keeps the grants mapped, so nothing the device may
//! still write to is handed back.

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::driver::Driver;
use crate::firmware::gen3::bringup::boot;
use crate::firmware::gen3::classify;
use crate::firmware::gen3::dev::Dev;
use crate::firmware::gen3::layout::{firmware_region_sizes, Board, Memory};
use crate::firmware::gen3::outcome::Failure;
use crate::firmware::gen3::plan::{control_len_pnvm, RB_REGION};
use crate::firmware::gen3::pnvm::reserve as pnvm_reserve;
use crate::firmware::gen3::prph::read_umac;
use crate::firmware::gen3::regs::{
    CSR_HW_REV, CSR_HW_RF_ID, CSR_MSIX_HW_INT_MASK_AD, UMAG_SB_CPU_1_STATUS, UMAG_SB_CPU_2_STATUS,
};
use crate::firmware::gen3::select::{mac_type, rf_type, select, transport, Refusal};
use crate::firmware::gen3::start::{rf_killed, start_hw, stop_device};
use crate::firmware::gen3::station::ids::check as check_join_api;
use crate::firmware::gen3::ucode::Ucode;
use crate::firmware::gen3::up::{up, SCAN_IF_ADDR};
use crate::firmware::{gen3_blob, gen3_pnvm};
use crate::regs::Regs;

use super::address::draw;
use super::clock::Uptime;
use super::grant::Grant;
use super::say::Line;

/// The register window must reach the MSI-X cause and mask registers.
const WINDOW_MIN: u64 = (CSR_MSIX_HW_INT_MASK_AD + 4) as u64;

/// What the card said it is, for the status reply, and whether this path
/// has touched it (from then on it owns the card).
#[derive(Clone, Copy, Default)]
pub struct Seen {
    pub hw_rev: u32,
    pub rf_id: u32,
    pub touched: bool,
}

/// The radio after a good bring-up.
pub struct Up {
    pub dev: Dev<'static, Regs, Grant>,
    pub channels: Vec<u8>,
    /// The station address drawn for this boot; `None` scans only.
    pub addr: Option<[u8; 6]>,
    /// The antennas both the firmware's PHY configuration and the NVM allow.
    pub tx_ant: u8,
    pub rx_ant: u8,
    /// The firmware runs every join command at the layout this driver
    /// encodes (`station::ids`).
    pub join_api: bool,
}

// Map the control region, the receive buffers and each firmware region, in
// that order; on any refusal give back what was mapped and say so.
fn map_all(d: &Driver, ctrl_len: usize, fw: &[usize]) -> Option<Vec<Grant>> {
    let mut out: Vec<Grant> = Vec::with_capacity(fw.len() + 2);
    for &len in [ctrl_len, RB_REGION].iter().chain(fw) {
        match Grant::map(d.device_id, d.claim_epoch, len) {
            Some(g) => out.push(g),
            None => {
                Line::new().text(b"no DMA region of ").num(len as u32).text(b" bytes").send();
                out.into_iter().for_each(Grant::unmap);
                return None;
            }
        }
    }
    Some(out)
}

pub fn bring_up(d: &Driver, ids: &mut Seen) -> Result<Up, Failure> {
    // An adapter that is not an SO platform is left exactly as setup left it.
    if transport(d.pci_device).is_none() {
        return Err(Failure::Refused(Refusal::NotSoDevice(d.pci_device)));
    }
    if d.mmio_len < WINDOW_MIN {
        return Err(Failure::Window);
    }
    ids.touched = true;
    let m = d.regs;
    let mut c = Uptime;
    ids.hw_rev = m.read32(CSR_HW_REV);
    if ids.hw_rev == u32::MAX {
        return Err(Failure::DeadMmio);
    }
    start_hw(&m, &mut c).map_err(Failure::Start)?;
    ids.rf_id = m.read32(CSR_HW_RF_ID);
    Line::new()
        .text(b"pci ")
        .hex(u32::from(d.pci_device))
        .text(b" hw_rev ")
        .hex(ids.hw_rev)
        .text(b" rf_id ")
        .hex(ids.rf_id)
        .send();

    let (image, transport) =
        select(d.pci_device, ids.hw_rev, ids.rf_id).map_err(Failure::Refused)?;
    let blob = gen3_blob(image).ok_or(Failure::Refused(Refusal::ImageNotBundled(image)))?;
    Line::new().text(b"firmware ").text(blob.name.as_bytes()).send();
    let ucode = Ucode::parse(blob.bytes).ok_or(Failure::Image)?;
    let fw = classify(blob.bytes);
    let sizes = firmware_region_sizes(&fw).ok_or(Failure::Image)?;
    // Space after the image loader for the largest platform NVM section this
    // adapter could be given; the SKU that picks one is only known at ALIVE.
    let pnvm = gen3_pnvm(image);
    let reserve = pnvm.map_or(0, |f| pnvm_reserve(f, mac_type(ids.hw_rev), rf_type(ids.rf_id)));
    let ctrl_len = control_len_pnvm(ucode.iml.len(), reserve).ok_or(Failure::Image)?;
    let grants = map_all(d, ctrl_len, &sizes).ok_or(Failure::NoDma)?;

    // From the kick on the device holds these addresses: the grants live as
    // long as the capsule does.
    let grants: &'static [Grant] = Vec::leak(grants);
    let (ctrl, rest) = grants.split_first().ok_or(Failure::NoDma)?;
    let (rbs, fw_grants) = rest.split_first().ok_or(Failure::NoDma)?;
    let fw_refs: &'static [&'static Grant] = Vec::leak(fw_grants.iter().collect());
    let regs: &'static Regs = Box::leak(Box::new(m));
    let mut dev = Dev::new(regs, ctrl, rbs);
    let mem = Memory { ctrl, rbs, fw: fw_refs };
    let board =
        Board { hw_rev: ids.hw_rev, imr_enabled: transport.imr_enabled, rf_id: ids.rf_id, pnvm };

    let alive = match boot(&mut dev, &mut c, &mem, &fw, &ucode, &board) {
        Ok(a) => a,
        Err(e) => {
            // What Linux prints when ALIVE does not come: how far the ROM's
            // secure boot got on each CPU.
            let cpu1 = read_umac(regs, &mut c, UMAG_SB_CPU_1_STATUS).unwrap_or(u32::MAX);
            let cpu2 = read_umac(regs, &mut c, UMAG_SB_CPU_2_STATUS).unwrap_or(u32::MAX);
            Line::new()
                .text(b"secure boot status: cpu1 ")
                .hex(cpu1)
                .text(b" cpu2 ")
                .hex(cpu2)
                .send();
            stop_device(regs, &mut c);
            return Err(Failure::Boot(e));
        }
    };
    Line::new()
        .text(b"firmware alive: status ")
        .hex(u32::from(alive.status))
        .text(b" umac ")
        .num(alive.umac_major)
        .text(b".")
        .num(alive.umac_minor)
        .text(b" sku ")
        .hex(alive.sku_id[0])
        .send();
    match (pnvm.is_some(), dev.pnvm) {
        (_, Some(v)) => Line::new().text(b"platform NVM section ").hex(v).text(b" given").send(),
        (true, None) if alive.sku_id != [0; 3] => {
            Line::new().text(b"no platform NVM section for this SKU: firmware defaults").send()
        }
        _ => {}
    }

    // The station address for this boot; without randomness the interface
    // takes the fixed one, which only a passive scan (sending nothing) uses.
    let addr = draw();
    if addr.is_none() {
        Line::new().text(b"no randomness for a station address: scanning only").send();
    }
    let info = match up(&mut dev, &mut c, &ucode, &transport, addr.unwrap_or(SCAN_IF_ADDR)) {
        Ok(i) => i,
        Err(e) => {
            stop_device(regs, &mut c);
            return Err(Failure::Up(e));
        }
    };
    let mut l = Line::new();
    l.text(b"up: ")
        .num(info.nvm.channels.len() as u32)
        .text(b" channels, tx ant ")
        .num(u32::from(info.nvm.valid_tx_ant));
    if let Some(mcc) = info.mcc {
        l.text(b", regulatory ").hex(u32::from(mcc));
    }
    l.send();
    if rf_killed(regs) {
        Line::new().text(b"the hardware RF-kill switch has the radio off").send();
    }
    let join_api = match check_join_api(&ucode) {
        Ok(()) => true,
        Err((group, cmd)) => {
            Line::new()
                .text(b"joining off: the firmware runs another layout of command ")
                .hex(u32::from(group))
                .text(b"/")
                .hex(u32::from(cmd))
                .send();
            false
        }
    };
    let (tx_ant, rx_ant) = (
        ucode.valid_tx_ant() & info.nvm.valid_tx_ant,
        ucode.valid_rx_ant() & info.nvm.valid_rx_ant,
    );
    Ok(Up { dev, channels: info.nvm.channels, addr, tx_ant, rx_ant, join_api })
}
