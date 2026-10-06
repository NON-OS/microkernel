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

//! The rect a window takes when its client resizes it.
//!
//! The client has already put its new surface in the scene at its own origin
//! when it tells the window manager, so the origin stays where it is and only
//! the size can be held back, to what is left of the display right of and
//! below it. This used to clamp the whole rect to the display, which moved
//! the origin left or up whenever the new size ran past an edge (a resize to
//! the right edge does, by the frame's margin): the window manager then hit
//! tested a window some pixels away from the one on screen, and every press
//! on it arrived that far off.
//!
//! The resize handler also refused any size that overlapped another window,
//! after the client had drawn it. Cascaded windows always overlap, so most
//! resizes left this table holding the old size: a press on the part the
//! window grew into went to the window under it, and one on the part it gave
//! up still went to it.

use super::constrain::{clamp_to_display, MIN_WINDOW_DIM};
use super::rect::Rect;

pub fn resized(current: Rect, w: u32, h: u32, display_w: u32, display_h: u32) -> Rect {
    let room_w = display_w.saturating_sub(current.x);
    let room_h = display_h.saturating_sub(current.y);
    if room_w < MIN_WINDOW_DIM || room_h < MIN_WINDOW_DIM {
        // No room right of or below the origin at all: only a moved window fits.
        return clamp_to_display(
            Rect { x: current.x, y: current.y, width: w, height: h },
            display_w,
            display_h,
        );
    }
    let width = w.clamp(MIN_WINDOW_DIM, room_w);
    let height = h.clamp(MIN_WINDOW_DIM, room_h);
    Rect { x: current.x, y: current.y, width, height }
}
