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
//! The part two rectangles share, if any.

use crate::state::damage::Rect;

pub fn meet(a: Rect, b: Rect) -> Option<Rect> {
    let (x0, y0) = (a.x.max(b.x), a.y.max(b.y));
    let x1 = (a.x + a.width).min(b.x.saturating_add(b.width));
    let y1 = (a.y + a.height).min(b.y.saturating_add(b.height));
    (x0 < x1 && y0 < y1).then(|| Rect { x: x0, y: y0, width: x1 - x0, height: y1 - y0 })
}
