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

//! Giving the device back, from any attempt that took DMA frames: both
//! queues and engines stood down, the master stop waited for and the part
//! reset, then Bus Master turned off, then the device released. The release
//! frees every grant tied to the device after the broker has read Bus Master
//! back off, so no frame is unmapped here: a queue left running could write
//! into ring memory the broker hands to someone else.

use nonos_libc::{mk_device_release, MmioMapOut};

use super::pci::command;
use crate::init::{reset, stand_down};
use crate::regs::Regs;

pub fn after(device_id: u64, epoch: u64, mmio: &MmioMapOut) {
    release(device_id, epoch, &Regs::new(mmio.user_va));
}

/// Best-effort throughout: a part whose master will not stop is not reset
/// (that is the hang the master stop exists for), but still goes back with
/// Bus Master off, and the broker's own stop runs before any frame is freed.
pub fn release(device_id: u64, epoch: u64, regs: &Regs) {
    stand_down::run(regs);
    let _ = reset::run(regs);
    let _ = command(device_id, epoch, false);
    let _ = mk_device_release(device_id);
}
