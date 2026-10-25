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

//! `reclaim`'s step under GATE (orphans.rs): the orphans no other process
//! maps any more, their slots released and their frames freed.

use alloc::vec::Vec;

use super::super::attach_map::snapshot::has_foreign_holder;
use super::super::release::release_unmapped;
use super::kept::pending;
use super::orphans::ORPHANS;

/* Under GATE: the orphans no other process maps, released and freed. */
pub(super) fn free_orphans() {
    let mut done = Vec::new();
    {
        let mut list = ORPHANS.lock();
        let mut i = 0;
        while i < list.len() {
            // A sibling still on the dying tables can write the frames (L49).
            if has_foreign_holder(list[i].handle, list[i].owner) || pending(list[i].tables) {
                i += 1;
            } else {
                done.push(list.swap_remove(i));
            }
        }
    }
    for orphan in done {
        // The owner gave the surface up when it unmapped all of it; now that
        // nobody maps it, its slot goes too.
        release_unmapped(orphan.owner, orphan.handle);
        for frame in orphan.free {
            if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {
                crate::sys::serial::println(b"[SURFACE] orphan_frame_release_failed");
            }
        }
    }
}
