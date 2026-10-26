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
//! The rect a drag of a window's edges leaves (toolkit decorations/reshape.rs):
//! only the edges held move, by the pointer's travel, and none passes the
//! least size or the top of the work area while the far edges stay.

use nonos_toolkit::decorations::{reshaped, Edges, Placed};

#[test]
fn a_drag_moves_only_the_edges_it_holds() {
    let base = Placed { x: 400, y: 200, w: 600, h: 400 };
    let edges = |l, r, t, b| Edges { left: l, right: r, top: t, bottom: b };
    let at = |e: Edges, d: (i64, i64)| reshaped(base, e, d, (200, 150), 46);
    assert_eq!(at(edges(false, true, false, false), (50, 9)), Placed { w: 650, ..base });
    assert_eq!(at(edges(true, false, false, false), (-50, 9)), Placed { x: 350, w: 650, ..base });
    assert_eq!(at(edges(false, false, true, false), (9, 30)), Placed { y: 230, h: 370, ..base });
    assert_eq!(at(edges(false, false, false, true), (9, -30)), Placed { h: 370, ..base });
}

#[test]
fn a_drag_stops_at_the_least_size_and_the_work_area_with_the_far_edge_still() {
    let base = Placed { x: 400, y: 200, w: 600, h: 400 };
    let tl = Edges { left: true, top: true, ..Edges::default() };
    let p = reshaped(base, tl, (2000, 2000), (200, 150), 46);
    assert_eq!(
        (p.x + p.w, p.y + p.h, p.w, p.h),
        (1000, 600, 200, 150),
        "least size, far edges kept"
    );
    let p = reshaped(base, tl, (-2000, -2000), (200, 150), 46);
    assert_eq!(
        (p.x, p.y, p.x + p.w, p.y + p.h),
        (0, 46, 1000, 600),
        "the screen's left, the work area's top"
    );
}
