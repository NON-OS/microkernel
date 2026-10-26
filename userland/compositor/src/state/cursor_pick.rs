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
//! Which shape the pointer takes: the one the window under it asked for
//! (OP_CURSOR_SHAPE), the arrow over anything that asked for none.

use super::scene::Layer;
use crate::frame_pacer::cursor_shapes::ARROW_CODE;

/// The shape of the topmost of `layers`, given bottom to top, that holds
/// (x, y) and has asked for one; the arrow when none has. The shell's chrome
/// covers the whole display over every window and asks for none, so it never
/// hides the shape of the window under its transparent pixels.
pub fn shape_at<'a>(layers: impl DoubleEndedIterator<Item = &'a Layer>, x: u32, y: u32) -> u8 {
    let holds = |l: &&Layer| x >= l.x && y >= l.y && x - l.x < l.width && y - l.y < l.height;
    layers.rev().filter(holds).find_map(|l| l.cursor).unwrap_or(ARROW_CODE)
}

/// How far any shape reaches above and left of its hot spot.
pub const HOT_REACH: u32 = 10;
/// The side of the square every shape fits in from HOT_REACH back.
pub const SHAPE_BOX: u32 = 32;

/// The pixels the pointer at (x, y) may cover, on a `w` by `h` canvas.
pub fn reach(x: u32, y: u32, w: u32, h: u32) -> crate::state::damage::Rect {
    let (x0, y0) = (x.saturating_sub(HOT_REACH), y.saturating_sub(HOT_REACH));
    let width = SHAPE_BOX.min(w.saturating_sub(x0));
    crate::state::damage::Rect { x: x0, y: y0, width, height: SHAPE_BOX.min(h.saturating_sub(y0)) }
}
