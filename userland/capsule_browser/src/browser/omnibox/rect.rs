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

//! Joining and placing the toolkit's rects, for the damage map.

use nonos_toolkit::widgets::Rect;

/// The smallest rect holding both `a` and `b`.
pub fn union(a: Rect, b: Rect) -> Rect {
    let (x, y) = (a.x.min(b.x), a.y.min(b.y));
    let r = (a.x + a.w).max(b.x + b.w);
    let bottom = (a.y + a.h).max(b.y + b.h);
    Rect { x, y, w: r - x, h: bottom - y }
}

/// `r`, given relative to `origin`, moved to where `origin` is.
pub fn within(r: Rect, origin: Rect) -> Rect {
    Rect { x: origin.x.saturating_add(r.x), y: origin.y.saturating_add(r.y), ..r }
}
