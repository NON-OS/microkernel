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

//! FOCUS_SET and CURSOR_UPDATE: who has the keyboard, and where the
//! pointer is drawn.

use super::read::wide;
use super::read::words;
use crate::protocol::{CURSOR_SHAPE_REQ_LEN, CURSOR_UPDATE_REQ_LEN, FOCUS_SET_REQ_LEN};

/// FOCUS_SET: the pid now focused; the padding word must be zero.
pub fn focus_set(body: &[u8]) -> Option<u32> {
    let [pid, pad] = words::<2, FOCUS_SET_REQ_LEN>(body)?;
    (pad == 0).then_some(pid)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Cursor {
    pub x: u32,
    pub y: u32,
    pub visible: bool,
}

/// CURSOR_UPDATE: any non-zero third word shows the pointer; the padding
/// word must be zero.
pub fn cursor_update(body: &[u8]) -> Option<Cursor> {
    let [x, y, visible, pad] = words::<4, CURSOR_UPDATE_REQ_LEN>(body)?;
    (pad == 0).then_some(Cursor { x, y, visible: visible != 0 })
}

/// The code that withdraws a surface's say in the shape: the pointer over it
/// takes the shape of what lies under it again (state/cursor_pick.rs).
pub const CURSOR_WITHDRAW: u32 = 0xFF;

/// The surface a shape is asked for and the shape, which must be one the
/// compositor draws (frame_pacer/cursor_shapes.rs), or None to withdraw.
pub fn cursor_shape(body: &[u8]) -> Option<(u64, Option<u8>)> {
    let [lo, hi, code, pad] = words::<4, CURSOR_SHAPE_REQ_LEN>(body)?;
    let shape = match code {
        CURSOR_WITHDRAW => None,
        c if c <= crate::frame_pacer::cursor_shapes::NESW_CODE as u32 => Some(c as u8),
        _ => return None,
    };
    (pad == 0).then_some((wide(lo, hi), shape))
}

impl Cursor {
    /// Whether the point is on a `w` by `h` canvas.
    pub fn on(&self, w: u32, h: u32) -> bool {
        self.x < w && self.y < h
    }
}
