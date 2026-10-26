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

//! Ending the RAM volume within a boot: when keeping turns on, the volume an
//! installed disk's was held in is dropped, and the disk's is opened instead
//! (zerostate/volume_rule.rs). Every frame is zeroed before it is given back.

use core::sync::atomic::Ordering;

use super::{base, slot, Ram, ON, RAM};
use crate::memory::phys::{free, Frame, PAGE_SIZE_U64};

/// Stop holding sectors in RAM. The volume's sectors go back to the disk's
/// device path; a RAM volume may be started again after.
pub fn stop() {
    let mut guard = RAM.lock();
    ON.store(false, Ordering::SeqCst);
    let Some(ram) = guard.take() else { return };
    release(&ram);
}

fn release(ram: &Ram) {
    for &leaf in ram.leaves.iter().filter(|&&leaf| leaf != 0) {
        for i in 0..(PAGE_SIZE_U64 / 8) as usize {
            let frame = slot(leaf, i);
            if frame != 0 {
                give_back(frame);
            }
        }
        give_back(leaf);
    }
}

fn give_back(frame: u64) {
    if let Some(p) = base(frame) {
        // SAFETY: as in ram.rs: a frame this module took, in the direct map,
        // under the RAM lock, and no longer named by the volume.
        unsafe { core::ptr::write_bytes(p, 0, PAGE_SIZE_U64 as usize) };
    }
    let _ = free(Frame(frame));
}
