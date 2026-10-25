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

//! xdg_positioner: where a popup goes, as its client describes it. Pure, so
//! the host proofs hold it. A size or anchor rectangle that is not positive,
//! or an anchor or gravity past the protocol's nine, is the client's
//! invalid_input, and the positioner keeps what it had.

/// set_constraint_adjustment's bits.
pub const SLIDE_X: u32 = 1;
pub const SLIDE_Y: u32 = 2;
pub const FLIP_X: u32 = 4;
pub const FLIP_Y: u32 = 8;
pub const RESIZE_X: u32 = 16;
pub const RESIZE_Y: u32 = 32;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct Positioner {
    pub size: (i32, i32),
    /// x, y, width, height, on the parent's surface.
    pub rect: (i32, i32, i32, i32),
    /// The protocol's anchor and gravity: 0 none, 1 top, 2 bottom, 3 left,
    /// 4 right, 5 top left, 6 bottom left, 7 top right, 8 bottom right.
    pub anchor: u32,
    pub gravity: u32,
    pub adjust: u32,
    pub offset: (i32, i32),
}

/// xdg_positioner request `op`, set_size (1) to set_parent_configure (9),
/// with its words; false when the request is invalid_input.
pub fn set(p: &mut Positioner, op: u16, w: &[u32]) -> bool {
    let i = |n: usize| w.get(n).map(|v| *v as i32);
    match (op, i(0), i(1), i(2), i(3)) {
        (1, Some(x), Some(y), _, _) if x > 0 && y > 0 => p.size = (x, y),
        (2, Some(x), Some(y), Some(ww), Some(h)) if ww >= 0 && h >= 0 => p.rect = (x, y, ww, h),
        (3, Some(a), ..) if (0..=8).contains(&a) => p.anchor = a as u32,
        (4, Some(g), ..) if (0..=8).contains(&g) => p.gravity = g as u32,
        (5, Some(a), ..) => p.adjust = a as u32,
        (6, Some(x), Some(y), _, _) => p.offset = (x, y),
        // set_reactive, set_parent_size, set_parent_configure: no effect here.
        (7..=9, ..) => {}
        _ => return false,
    }
    true
}
