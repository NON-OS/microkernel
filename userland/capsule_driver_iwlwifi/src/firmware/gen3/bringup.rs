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

//! From claimed chip to running firmware, as Linux v6.12 runs
//! `iwl_mvm_load_ucode_wait_alive` for a unified image: start the hardware,
//! lay out memory, kick the self-load, wait for the ALIVE cause (2 s,
//! `MVM_UCODE_ALIVE_TIMEOUT`), restock the receive ring (the firmware has
//! configured it by then), read the ALIVE notification and check its status,
//! then, if the part reports a SKU id, ring the platform-NVM doorbell and wait
//! for its completion (250 ms, `MVM_UCODE_PNVM_TIMEOUT`). Before the doorbell
//! the SKU's platform NVM section, when the image's file is carried, is laid
//! out after the image loader and named in the peripheral scratch (`pnvm`).
//! Without the file, or with no section for this adapter, the doorbell rings
//! anyway and the firmware runs on its defaults, which Linux also tolerates.

use super::alive::{parse as parse_alive, Alive, UCODE_ALIVE_NTFY};
use super::cmds::{LEGACY_GROUP, LONG_GROUP, PNVM_INIT_COMPLETE_NTFY, REGULATORY_AND_NVM_GROUP};
use super::dev::{Dev, Flow, WaitError};
use super::layout::{lay_out, Board, LayoutError, Memory};
use super::packet::Packet;
use super::plan::{pnvm_off, CTXT_INFO, IML, PRPH_SCRATCH};
use super::pnvm;
use super::prph::write_umac;
use super::region::{Clock, Region};
use super::regs::{
    CSR_INT, CSR_MSIX_HW_INT_CAUSES_AD, INT_BIT_ALIVE, MSIX_HW_ALIVE, UREG_DOORBELL_TO_ISR6,
    UREG_DOORBELL_TO_ISR6_PNVM,
};
use super::select::{mac_type, rf_type};
use super::start::{
    after_kick, before_kick, enable_alive_int, mask_interrupts, start_hw, StartError,
};
use super::ucode::Ucode;
use super::{kick, FwLayout};
use crate::regs::Mmio;

pub const ALIVE_MS: u32 = 2000;
pub const PNVM_MS: u32 = 250;

/// How far a failed boot got.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BootError {
    Start(StartError),
    Layout(LayoutError),
    /// The ALIVE cause never came.
    NoAliveCause,
    /// The cause came but no ALIVE notification followed.
    NoAliveNotif(WaitError),
    /// The notification's status was not 0xCAFE.
    NotAlive(u16),
    /// The platform NVM step did not complete.
    Pnvm(WaitError),
    NoNicAccess,
}

/// Legacy notifications arrive in group 0 or, with the wide header, group 1.
pub fn is_legacy(p: &Packet<'_>, cmd: u8) -> bool {
    p.cmd == cmd && (p.group == LEGACY_GROUP || p.group == LONG_GROUP)
}

// Lay this SKU's platform NVM section out after the image loader and point
// the peripheral scratch at it; its version, or `None` when the file has no
// section for this adapter or it does not fit the reserved space.
fn place_pnvm<R: Region + ?Sized>(
    ctrl: &R,
    file: &[u8],
    sku: [u32; 3],
    board: &Board,
    iml_len: usize,
) -> Option<u32> {
    let p = pnvm::parse(file, sku, mac_type(board.hw_rev), rf_type(board.rf_id))?;
    let placed = pnvm::lay_out(ctrl, pnvm_off(iml_len)?, &p)?;
    pnvm::point_scratch(ctrl, PRPH_SCRATCH, placed).then_some(p.version)
}

/// Boot the firmware and return its ALIVE.
pub fn boot<M: Mmio, R: Region + ?Sized, C: Clock>(
    dev: &mut Dev<'_, M, R>,
    c: &mut C,
    mem: &Memory<'_, R>,
    fw: &FwLayout<'_>,
    ucode: &Ucode<'_>,
    board: &Board,
) -> Result<Alive, BootError> {
    let m = dev.m;
    start_hw(m, c).map_err(BootError::Start)?;
    before_kick(m, c).map_err(BootError::Start)?;
    lay_out(mem, fw, ucode.iml, board, &mut dev.rxq, &mut dev.cmdq).map_err(BootError::Layout)?;
    enable_alive_int(m);
    let base = mem.ctrl.dev();
    kick(m, base + CTXT_INFO as u64, base + IML as u64, ucode.iml.len() as u32);
    after_kick(m, c).map_err(BootError::Start)?;

    let mut cause = || {
        m.read32(CSR_INT) & INT_BIT_ALIVE != 0
            || m.read32(CSR_MSIX_HW_INT_CAUSES_AD) & MSIX_HW_ALIVE != 0
    };
    if !c.poll_for(ALIVE_MS, &mut cause) {
        return Err(BootError::NoAliveCause);
    }
    m.write32(CSR_INT, INT_BIT_ALIVE);
    m.write32(CSR_MSIX_HW_INT_CAUSES_AD, MSIX_HW_ALIVE);
    mask_interrupts(m);
    dev.rxq.kick(m);

    let ver = ucode.notif_version(LEGACY_GROUP, UCODE_ALIVE_NTFY);
    let mut alive = None;
    dev.wait(c, ALIVE_MS, &mut |p| {
        if is_legacy(p, UCODE_ALIVE_NTFY) {
            alive = parse_alive(p.payload, ver);
            if alive.is_some() {
                return Flow::Done;
            }
        }
        Flow::Continue
    })
    .map_err(BootError::NoAliveNotif)?;
    let alive = alive.ok_or(BootError::NoAliveNotif(WaitError::TimedOut))?;
    if !alive.ok() {
        return Err(BootError::NotAlive(alive.status));
    }

    if alive.sku_id != [0; 3] {
        // `iwl_pnvm_load_pnvm_to_trans`: give the firmware this SKU's section
        // when the tree carries the file. Without it, or with no matching
        // section, the doorbell still rings and the firmware runs on its
        // defaults, as Linux's does.
        if let Some(file) = board.pnvm {
            dev.pnvm = place_pnvm(mem.ctrl, file, alive.sku_id, board, ucode.iml.len());
        }
        if !write_umac(m, c, UREG_DOORBELL_TO_ISR6, UREG_DOORBELL_TO_ISR6_PNVM) {
            return Err(BootError::NoNicAccess);
        }
        dev.wait(c, PNVM_MS, &mut |p| {
            if p.group == REGULATORY_AND_NVM_GROUP && p.cmd == PNVM_INIT_COMPLETE_NTFY {
                Flow::Done
            } else {
                Flow::Continue
            }
        })
        .map_err(BootError::Pnvm)?;
    }
    Ok(alive)
}
