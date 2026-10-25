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

//! The canvas onto a screen at `q` quarters (4 one to one, 5 and 6 the
//! fractional steps, 8 doubled): whole steps through `upscale`, the rest
//! through `stretch`.

use super::{stretch::stretch, upscale, Surface};
use crate::state::damage::Rect;

pub fn scaled(canvas: Surface, screen: Surface, rect: Rect, q: u32) -> Option<Rect> {
    if q.is_multiple_of(4) {
        upscale(canvas, screen, rect, q / 4)
    } else {
        stretch(canvas, screen, rect, q)
    }
}
