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

//! Giving the device back, from any attempt that took DMA frames: the part's
//! DMA stopped as before every reset, then Bus Master turned off so the reset
//! lands with the part unable to reach memory (as `reset_part` orders them),
//! then the reset (but see `after`), then the device released. The release
//! frees every grant tied to the device after the broker has read Bus Master
//! back off, so no frame is unmapped here: a receiver left running would
//! write wire frames into memory the broker hands to someone else.

use nonos_libc::{mk_device_release, MmioMapOut};

use super::pci::command;
use crate::constants::Family;
use crate::init::{reset, stop_dma, stop_units};
use crate::regs::Regs;

/// From the setup sequence, before any ring of this attempt was programmed.
/// On pch_spt and later the reset is left out: the rings firmware left may
/// still hold descriptors, a reset on them is a unit hang only a PCI reset
/// clears, and the flush that clears them (Linux e1000_flush_desc_rings, as
/// the teardown runs it) runs only on this driver's rings. Both units
/// stopped keep the part off memory until the next attempt resets it with
/// Bus Master off (`reset_part`).
/// Older parts are stopped as Linux stops them before its reset.
pub fn after(device_id: u64, epoch: u64, mmio: &MmioMapOut, family: Family) {
    let regs = Regs::new(mmio.user_va);
    let reset_as = if family < Family::PchSpt {
        stop_dma(&regs, family);
        Some(family)
    } else {
        stop_units(&regs, family);
        None
    };
    release(device_id, epoch, &regs, reset_as);
}

/// Once the part's DMA is stopped (`stop_dma`, or the quiesce at teardown):
/// Bus Master off, the reset for the family `reset_as` names (none for
/// None), then the device back. Best-effort throughout: a part that will not
/// reset still goes back, and the broker's own stop runs before any frame
/// is freed.
pub fn release(device_id: u64, epoch: u64, regs: &Regs, reset_as: Option<Family>) {
    let _ = command(device_id, epoch, false);
    if let Some(family) = reset_as {
        let _ = reset::run(regs, family);
    }
    let _ = mk_device_release(device_id);
}
