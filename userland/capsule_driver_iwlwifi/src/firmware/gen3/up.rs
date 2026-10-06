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

//! After ALIVE, the commands that make a unified firmware ready to scan, in
//! Linux v6.12 order: `iwl_run_unified_mvm_ucode` (INIT_EXTENDED_CFG,
//! NVM_ACCESS_COMPLETE, wait for INIT_COMPLETE, NVM_GET_INFO), the parts of
//! `iwl_mvm_up` a scan rests on (TX antennas, BT coexistence, SoC latency,
//! device power, the regulatory domain, scan config), and the interface add
//! (`iwl_mvm_mld_mac_add_interface`: MAC_CONFIG and LINK_CONFIG, this
//! firmware being MLD-API), at the station address the caller drew. Optional platform tables Linux reads from the BIOS
//! (SAR, PPAG, TAS, LARI) are absent here, as on a machine without them. A
//! command the firmware rejects or does not answer stops the sequence and
//! names itself.

use super::bringup::is_legacy;
use super::cmds::*;
use super::dev::{Dev, Flow, WaitError, COMMAND_MS};
use super::nvm::{parse as parse_nvm, NvmInfo};
use super::packet::Packet;
use super::region::{Clock, Region};
use super::select::Transport;
use super::ucode::Ucode;
use crate::regs::Mmio;

/// The address the interface is created with when the kernel gave no
/// randomness for a station address. Only a passive scan runs then, which
/// sends nothing, so it never reaches the air; the device's own address is
/// not handed out at all.
pub const SCAN_IF_ADDR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum UpError {
    /// A command was not answered (or the firmware failed meanwhile).
    Command(u8, u8, WaitError),
    /// INIT_COMPLETE never came.
    NoInitComplete(WaitError),
    /// The NVM reply was malformed.
    BadNvm,
    /// The MCC reply was malformed.
    BadMcc,
    /// The firmware wants an API this driver does not speak.
    Unsupported(&'static str),
}

/// What the firmware told the driver while coming up.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct UpInfo {
    pub nvm: NvmInfo,
    /// The regulatory domain the firmware chose, when LAR is on.
    pub mcc: Option<u16>,
}

// Run one command; INIT_COMPLETE arriving meanwhile is remembered.
fn run<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    id: (u8, u8),
    payload: &[u8],
    init_done: &mut bool,
    reply: &mut dyn FnMut(&[u8]),
) -> Result<(), UpError> {
    let mut other = |p: &Packet<'_>| {
        if is_legacy(p, INIT_COMPLETE_NOTIF) && !p.is_reply() {
            *init_done = true;
        }
    };
    dev.command(c, id, payload, reply, &mut other).map_err(|e| UpError::Command(id.0, id.1, e))
}

/// Bring the firmware up to the point a scan can run, with the interface at
/// `addr`.
pub fn up<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    ucode: &Ucode<'_>,
    transport: &Transport,
    addr: [u8; 6],
) -> Result<UpInfo, UpError> {
    if !ucode.capa(CAPA_MLD_API_SUPPORT) {
        return Err(UpError::Unsupported("firmware without the MLD API"));
    }
    if ucode.cmd_version(LONG_GROUP, SCAN_REQ_UMAC) != Some(17) {
        return Err(UpError::Unsupported("scan request version other than 17"));
    }
    let mut init_done = false;
    let mut ignore = |_: &[u8]| {};
    run(dev, c, (SYSTEM_GROUP, INIT_EXTENDED_CFG_CMD), &init_extended_cfg(), &mut init_done, &mut ignore)?;
    run(dev, c, (REGULATORY_AND_NVM_GROUP, NVM_ACCESS_COMPLETE), &reserved_word(), &mut init_done, &mut ignore)?;
    if !init_done {
        dev.wait(c, COMMAND_MS, &mut |p| {
            if is_legacy(p, INIT_COMPLETE_NOTIF) && !p.is_reply() {
                Flow::Done
            } else {
                Flow::Continue
            }
        })
        .map_err(UpError::NoInitComplete)?;
    }

    let v4 = ucode.api(API_REGULATORY_NVM_INFO);
    let mut nvm = None;
    run(dev, c, (REGULATORY_AND_NVM_GROUP, NVM_GET_INFO), &reserved_word(), &mut init_done, &mut |r| {
        nvm = parse_nvm(r, v4)
    })?;
    let nvm = nvm.ok_or(UpError::BadNvm)?;
    let tx_ant = ucode.valid_tx_ant() & nvm.valid_tx_ant;
    let rx_ant = ucode.valid_rx_ant() & nvm.valid_rx_ant;

    run(dev, c, (LONG_GROUP, TX_ANT_CONFIGURATION_CMD), &tx_ant_cfg(tx_ant), &mut init_done, &mut ignore)?;
    let bt = bt_config(ucode.capa(CAPA_BT_MPLUT_SUPPORT));
    run(dev, c, (LONG_GROUP, BT_CONFIG), &bt, &mut init_done, &mut ignore)?;
    if ucode.capa(CAPA_SOC_LATENCY_SUPPORT) {
        let t = transport;
        let soc = soc_configuration(t.integrated, t.ltr_delay, t.low_latency_xtal, t.xtal_latency);
        run(dev, c, (SYSTEM_GROUP, SOC_CONFIGURATION_CMD), &soc, &mut init_done, &mut ignore)?;
    }
    run(dev, c, (LONG_GROUP, POWER_TABLE_CMD), &device_power(), &mut init_done, &mut ignore)?;

    let mut mcc = None;
    if nvm.lar_enabled && ucode.capa(CAPA_LAR_SUPPORT) {
        let ver = ucode.notif_version(LONG_GROUP, MCC_UPDATE_CMD).unwrap_or(0);
        let ax = ucode.capa(CAPA_MCC_UPDATE_11AX_SUPPORT);
        let mut parsed = None;
        run(dev, c, (LONG_GROUP, MCC_UPDATE_CMD), &mcc_update_world(), &mut init_done, &mut |r| {
            parsed = mcc_update_reply(r, ver, ax)
        })?;
        mcc = Some(parsed.ok_or(UpError::BadMcc)?.1);
    }
    if ucode.capa(CAPA_UMAC_SCAN) {
        run(dev, c, (LONG_GROUP, SCAN_CFG_CMD), &scan_config(tx_ant, rx_ant), &mut init_done, &mut ignore)?;
    }
    run(dev, c, (MAC_CONF_GROUP, MAC_CONFIG_CMD), &mac_config_add(addr), &mut init_done, &mut ignore)?;
    run(dev, c, (MAC_CONF_GROUP, LINK_CONFIG_CMD), &link_config_add(addr), &mut init_done, &mut ignore)?;
    Ok(UpInfo { nvm, mcc })
}
