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

use alloc::vec::Vec;
use spin::Mutex;

use crate::memory::addr::PhysAddr;
use crate::smp::lock_responsive;

use super::super::attach_map::snapshot::has_foreign_holder;
use super::super::types::SurfaceHandle;
use super::gate::GATE;

/* An orphan: its frames, by page index, for matching mappings, and the ones
 * the owner really unmapped, freed by `reclaim` (dma_rule.rs). */
pub(super) type Orphan = super::dma_rule::PageOrphan<SurfaceHandle, PhysAddr>;

/* Frames of surfaces the owner gave up while another process still had them
 * attached, the registry's until no attach record is left. */
pub(super) static ORPHANS: Mutex<Vec<Orphan>> = Mutex::new(Vec::new());

/* Free every orphaned frame list that no other process maps any more. */
pub fn reclaim() {
    let _gate = lock_responsive(&GATE);
    let mut done = Vec::new();
    {
        let mut list = ORPHANS.lock();
        let mut i = 0;
        while i < list.len() {
            if has_foreign_holder(list[i].handle, list[i].owner) {
                i += 1;
            } else {
                done.push(list.swap_remove(i));
            }
        }
    }
    for orphan in done {
        // The owner gave the surface up when it unmapped all of it; now that
        // nobody maps it, its slot goes too.
        crate::kernel_core::surface_registry::release::release_unmapped(
            orphan.owner,
            orphan.handle,
        );
        for frame in orphan.free {
            if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {
                crate::sys::serial::println(b"[SURFACE] orphan_frame_release_failed");
            }
        }
    }
    super::dma_held::reclaim_dma(); // and the DMA grants held for them
}

/* `pid` exits. Its orphans stay listed: the other holders' attach records
 * outlive the owner, so `reclaim` frees each when its last holder lets go. */
pub fn abandon_owner(_pid: u32) {
    reclaim();
}
