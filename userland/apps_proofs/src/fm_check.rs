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

//! What every file manager control test asks of a table of drawn boxes: each
//! names its own target at its centre and its four inner corners, and no two
//! share a pixel; and the hover fed as `app.rs` feeds it.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, PointerAbs};
use nonos_toolkit::widgets::Rect;

use crate::fm::browse_hit::browse_at;
use crate::fm::hover::Hover;
use crate::fm::pane::Pane;
use crate::fm::target::Target;

/// The centre of `r`, then its four inner corners.
pub fn points(r: Rect) -> [(u32, u32); 5] {
    let (x1, y1) = (r.x + r.w - 1, r.y + r.h - 1);
    [(r.x + r.w / 2, r.y + r.h / 2), (r.x, r.y), (x1, r.y), (r.x, y1), (x1, y1)]
}

/// Whether two boxes share no pixel.
pub fn apart(a: &Rect, b: &Rect) -> bool {
    a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y
}

/// Checks the table `drawn` against `at`, which names what is at a point.
pub fn named_apart(drawn: &[(Target, Rect)], at: impl Fn(u32, u32) -> Option<Target>, what: &str) {
    for (i, &(t, r)) in drawn.iter().enumerate() {
        for (x, y) in points(r) {
            assert_eq!(at(x, y), Some(t), "({x}, {y}) in {what}");
        }
        assert!(drawn[i + 1..].iter().all(|(_, o)| apart(&r, o)), "{t:?} overlaps in {what}");
    }
}

/// What `(x, y)` names in the listing; nothing off the window's top left.
pub fn at(p: &Pane, x: i32, y: i32) -> Option<Target> {
    (x >= 0 && y >= 0).then(|| browse_at(p, x as u32, y as u32)).flatten()
}

/// The router's leave: a pointer at no position.
pub const LEAVE: (u32, u32) = (u32::MAX, u32::MAX);

/// Feeds the pointer, then names the target under it again; true to repaint.
pub fn feed(h: &mut Hover, p: &Pane, kind: InputKind, (x, y): (u32, u32)) -> bool {
    let (x, y) = if kind == PointerAbs && x == u32::MAX { (-1, -1) } else { (x as i32, y as i32) };
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    let held = h.feed(&InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns });
    h.settle(h.pointer.position().and_then(|(x, y)| at(p, x, y))) || held
}
