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
//! Which edges a press grabs (toolkit decorations/edges.rs), at every scale:
//! every edge and corner grabs, the controls and the rest of the bar never
//! do, and a maximised window has none.

use nonos_toolkit::decorations::{control_rect_at, edges_at, titlebar_rect_at, Edges, CONTROLS};

const W: u32 = 900;
const H: u32 = 600;
const SCALES: [u32; 4] = [4, 5, 6, 8];

#[test]
fn every_edge_and_corner_grabs_at_every_scale() {
    for q in SCALES {
        let e = |x, y| edges_at(W, H, false, x, y, q);
        let mid = (W / 2, H / 2);
        assert_eq!(e(2, mid.1), Edges { left: true, ..Edges::default() }, "left at {q}");
        assert_eq!(e(W - 3, mid.1), Edges { right: true, ..Edges::default() }, "right at {q}");
        assert_eq!(e(mid.0, 2), Edges { top: true, ..Edges::default() }, "top at {q}");
        assert_eq!(e(mid.0, H - 3), Edges { bottom: true, ..Edges::default() }, "bottom at {q}");
        assert!(e(2, 2).left && e(2, 2).top, "the top-left corner at {q}");
        assert!(e(W - 3, H - 3).right && e(W - 3, H - 3).bottom, "the bottom-right at {q}");
        assert!(!e(mid.0, mid.1).any(), "the middle is the app's at {q}");
        assert_eq!(edges_at(W, H, true, 2, 2, q), Edges::default(), "a maximised window has none");
    }
}

#[test]
fn the_controls_and_the_rest_of_the_bar_never_grab() {
    for q in SCALES {
        for i in 0..CONTROLS.len() as u32 {
            let r = control_rect_at(i, W, H, false, q);
            for (x, y) in
                [(r.x, r.y), (r.x + r.w - 1, r.y + r.h - 1), (r.x + r.w / 2, r.y + r.h / 2)]
            {
                assert!(!edges_at(W, H, false, x, y, q).any(), "control {i} at {x},{y} at {q}");
            }
        }
        let bar = titlebar_rect_at(W, H, false, q);
        let below_top = control_rect_at(0, W, H, false, q).y;
        for y in below_top..bar.y + bar.h {
            assert!(!edges_at(W, H, false, W / 2, y, q).any(), "the bar moves the window at {y}");
        }
    }
}
