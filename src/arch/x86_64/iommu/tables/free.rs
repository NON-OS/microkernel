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

//! A destroyed domain's tables back to the frame allocator, once
//! every unit dropped its cached translations: no walk can land in them
//! after that, so a reused frame is never read as a table.

use super::frame::entries_mut;
use super::frames_of::table_frames;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc::manager::deallocate_frame;

/// False, and nothing given back, when a table cannot be read.
pub fn free_tables(root: u64, levels: u8, child: impl Fn(u64, u8) -> Option<u64>) -> bool {
    let read = |table: u64| entries_mut(table).ok().map(|e| &*e);
    let Some(frames) = table_frames(root, levels, &read, &child) else {
        return false;
    };
    for frame in frames {
        let _ = deallocate_frame(PhysAddr::new(frame));
    }
    true
}
