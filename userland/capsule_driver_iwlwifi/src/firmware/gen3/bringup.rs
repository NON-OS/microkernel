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
use super::cmds::{LEGACY_GROUP, LONG_GROUP};
use super::dev::{Dev, Flow, WaitError};
use super::layout::{Board, LayoutError, Memory};
use super::packet::Packet;
use super::region::{Clock, Region};
use super::regs::{CSR_INT, CSR_MSIX_HW_INT_CAUSES_AD, INT_BIT_ALIVE, MSIX_HW_ALIVE};
use super::start::{mask_interrupts, StartError};
use super::ucode::Ucode;
use super::FwLayout;
use crate::regs::Mmio;
use kick::start_and_kick;
use pnvm_step::pnvm_step;

mod kick;
mod pnvm_step;

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
    start_and_kick(dev, c, mem, fw, ucode, board)?;

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
        pnvm_step(dev, c, mem.ctrl, board, ucode.iml.len(), alive.sku_id)?;
    }
    Ok(alive)
}
