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

//! Whether a dead domain's tables can all be read before any goes back to
//! the frame allocator (K12): one that cannot be read keeps them all, so no
//! frame a unit may still reach through it is ever reused. VT-d and AMD-Vi
//! both ask, each with its own rule for which entry leads to a table.

extern crate alloc;

use alloc::vec::Vec;

use super::frames_of::table_frames;
use super::sl_pte::ENTRIES;
use super::view::Frame;

/// True when every table under `root` (`levels` deep) reads whole.
pub fn all_readable(root: u64, levels: u8, child: &impl Fn(u64, u8) -> Option<u64>) -> bool {
    let read = |table: u64| -> Option<Vec<u64>> {
        let frame = Frame::at(table).ok()?;
        (0..ENTRIES).map(|index| frame.read(index).ok()).collect()
    };
    table_frames(root, levels, &read, child).is_some()
}
