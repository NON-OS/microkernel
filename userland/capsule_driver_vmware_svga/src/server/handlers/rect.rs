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

//! The rectangle a request names, checked against the screen.

use crate::protocol::le_u32;
use crate::screen::{rect_fits, Rect};
use crate::setup::Driver;

/// Four u32s at `at`: x, y, width, height; None unless they fit the screen.
pub fn rect_at(d: &Driver, body: &[u8], at: usize) -> Option<Rect> {
    let r = Rect {
        x: le_u32(body, at)?,
        y: le_u32(body, at + 4)?,
        w: le_u32(body, at + 8)?,
        h: le_u32(body, at + 12)?,
    };
    rect_fits(r, d.fb.width, d.fb.height).then_some(r)
}
