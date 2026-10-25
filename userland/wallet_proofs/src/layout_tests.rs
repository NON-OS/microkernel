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

//! The wallet's frame in a small window and at full screen: the column takes
//! the window less a margin up to a readable width, the type grows with it,
//! a wide window is two columns, and content never meets the edges.

use super::layout_frame::{GAP, ROOM, SIDE};
use crate::layout::{content, content_bottom, halves, layout, MAX_COLUMN};

#[test]
fn a_small_window_is_one_column_inside_its_margins() {
    let l = layout(800);
    assert_eq!(l.column, 680, "the photographs' width and half of the rest");
    assert_eq!((l.x, 800 - l.x - l.column), (60, 60), "the margins are equal");
    assert!(!l.two_up);
    assert_eq!(l.scale_pct, 100);
    let (cx, cw) = content(&l, SIDE);
    assert_eq!((cx, cw), (84, 632));
    assert!(cx >= l.x + SIDE && cx + cw <= l.x + l.column - SIDE);
    assert_eq!(content_bottom(600, 44, ROOM), 528, "room above a status-only foot");
}

#[test]
fn full_screen_is_two_columns_at_a_readable_width_with_larger_type() {
    let l = layout(1920);
    assert_eq!(l.column, MAX_COLUMN);
    assert_eq!(l.x, (1920 - MAX_COLUMN) / 2, "centred");
    assert!(l.two_up);
    assert_eq!(l.scale_pct, 125);
    let (cx, cw) = content(&l, SIDE);
    let ((lx, lw), (rx, rw)) = halves(cx, cw, 2 * GAP);
    assert_eq!(lx, cx);
    assert_eq!(rx + rw, cx + cw, "the right column ends where the content does");
    assert_eq!(rx - (lx + lw), 2 * GAP, "the gap between them is kept");
    assert!(lw.abs_diff(rw) <= 1, "the two columns are the same width");
    assert!(lw >= 400, "each column holds a tile of balances");
    assert_eq!(content_bottom(1080, 44, ROOM), 1008);
}
