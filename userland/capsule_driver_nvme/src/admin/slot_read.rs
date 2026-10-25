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


//! A completion ring slot read in the order entry_read sets, with the DMA
//! read barrier between the two loads.

use core::mem::offset_of;
use core::ptr::read_volatile;

use nonos_libc::dma_rmb;

use super::completion::Completion;
use super::entry_read::read_entry;

/// The entry at `slot`.
///
/// # Safety
/// `slot` is the address of a completion entry in a mapped DMA ring.
pub unsafe fn read_slot(slot: u64) -> Completion {
    let status = (slot as usize + offset_of!(Completion, status)) as *const u16;
    read_entry(
        // SAFETY: the status half-word lies inside the caller's slot.
        || unsafe { read_volatile(status) },
        dma_rmb,
        // SAFETY: the caller's slot.
        || unsafe { read_volatile(slot as *const Completion) },
    )
}
