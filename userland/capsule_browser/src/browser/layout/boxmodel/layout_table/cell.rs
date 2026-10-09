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

use super::super::display_list::DisplayList;
use super::super::shift_down::shift_down;

/* A cell laid out whose row span has not ended: its element, alignment
 * (halves of the slack above its content), top, content height,
 * fragments and last row. */
pub(super) struct Open {
    pub id: usize,
    pub valign: i32,
    pub top: i32,
    pub h: i32,
    pub frags: (usize, usize),
    pub last: u32,
}

/* Stretch cell `p`'s box to its slot `slot_h` tall and align its content
 * there. Always true: the cell is done. */
pub(super) fn settle(
    p: &Open,
    slot_h: i32,
    frags: &mut DisplayList,
    clip: Option<[i32; 4]>,
) -> bool {
    if let Some(f) = frags.get_mut(p.frags.0).filter(|f| f.node == p.id) {
        f.h = f.h.max(slot_h);
    }
    let dy = (slot_h - p.h) * p.valign / 2;
    shift_down(frags, p.frags.0 + 1, p.frags.1, dy.max(0), clip);
    true
}
