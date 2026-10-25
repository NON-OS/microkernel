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

//! Mirrored outputs in their own colours. Every mirror shows the one canvas
//! fitted. While each output's colour settings (filter, high contrast) are
//! output 0's, the canvas carries output 0's look and the mirrors copy the
//! screen. Once a mirror's differ, the canvas is kept in true colour and
//! each output's look is laid over its own copy, all but the indicator
//! band, which keeps its true colours everywhere. Night light is one for
//! every output, so it never differs. Pure.

use super::access_rule::Access;
use crate::state::damage::Rect;

/// Whether some mirrored output (of `heads`, after output 0) is to show
/// other colours than output 0's.
pub fn own_colours(extended: bool, access: &[Access], heads: usize) -> bool {
    let colour = |a: &Access| (a.filter, a.contrast);
    let first = access.first().map(colour);
    !extended && access.iter().skip(1).take(heads).any(|a| Some(colour(a)) != first)
}

/// `rect` less `hole`: the parts above, below, left and right of it.
pub fn around(rect: Rect, hole: Option<Rect>) -> [Option<Rect>; 4] {
    let Some(h) = hole.and_then(|h| meet(rect, h)) else { return [Some(rect), None, None, None] };
    let (rx1, ry1, hx1, hy1) =
        (rect.x + rect.width, rect.y + rect.height, h.x + h.width, h.y + h.height);
    let part = |x: u32, y: u32, x1: u32, y1: u32| {
        (x < x1 && y < y1).then(|| Rect { x, y, width: x1 - x, height: y1 - y })
    };
    [
        part(rect.x, rect.y, rx1, h.y),
        part(rect.x, hy1, rx1, ry1),
        part(rect.x, h.y, h.x, hy1),
        part(hx1, h.y, rx1, hy1),
    ]
}

fn meet(a: Rect, b: Rect) -> Option<Rect> {
    let (x0, y0) = (a.x.max(b.x), a.y.max(b.y));
    let x1 = (a.x + a.width).min(b.x + b.width);
    let y1 = (a.y + a.height).min(b.y + b.height);
    (x0 < x1 && y0 < y1).then(|| Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
}
