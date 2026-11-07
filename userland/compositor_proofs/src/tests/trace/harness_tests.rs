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

//! The harness composes what the scene changed and nothing else: the pixels
//! each frame repaints are the exact damage of its scene.

use crate::bench::run::{csv, run};
use crate::bench::scenes::{button_rect, caret_rect, under_rect, window_rect, Kind};
use crate::damage::Rect;

const W: u32 = 480;
const H: u32 = 270;

#[test]
fn a_caret_blink_repaints_one_glyph_cell() {
    let c = caret_rect(W, H);
    for r in run(Kind::Caret, W, H, 6) {
        assert_eq!((r.rects, r.pixels), (1, c.area()));
    }
}

#[test]
fn a_scroll_repaints_the_window_only() {
    let w = window_rect(W, H);
    for r in run(Kind::Scroll, W, H, 4) {
        assert_eq!((r.rects, r.pixels), (1, w.area()));
    }
}

#[test]
fn a_drag_repaints_where_the_window_was_and_is() {
    let base = window_rect(W, H);
    let (dx, dy) = ((W / 240).max(1), (H / 270).max(1));
    let recs = run(Kind::Drag, W, H, 5);
    for (i, r) in recs.iter().enumerate().skip(1) {
        let i = i as u32;
        let was = Rect { x: base.x + (i - 1) * dx, y: base.y + (i - 1) * dy, ..base };
        let now = Rect { x: base.x + i * dx, y: base.y + i * dy, ..base };
        assert_eq!(r.pixels, was.union(now).area(), "frame {i}");
    }
}

#[test]
fn a_pointer_move_repaints_less_than_two_shapes() {
    for r in run(Kind::Cursor, W, H, 8) {
        assert!(r.pixels > 0);
        assert!(r.pixels <= 2 * 32 * 32, "{} pixels", r.pixels);
    }
}

/// The pixels the arrow's box takes, twice: where it was and where it is.
const POINTER: u64 = 2 * 32 * 32;

#[test]
fn a_resize_repaints_the_window_at_its_old_and_new_sizes_only() {
    let base = window_rect(W, H);
    let recs = run(Kind::Resize, W, H, 5);
    for (i, r) in recs.iter().enumerate().skip(1) {
        let i = i as u32;
        let at = |k: u32| Rect { width: base.width + 8 * k, height: base.height + 8 * k, ..base };
        let both = at(i - 1).union(at(i)).area();
        assert!(r.pixels >= both && r.pixels <= both + POINTER, "frame {i}: {}", r.pixels);
    }
}

#[test]
fn a_hover_repaints_the_button_and_the_pointer_only() {
    let b = button_rect(window_rect(W, H), H);
    for r in run(Kind::Hover, W, H, 6) {
        assert!(r.pixels >= b.area() && r.pixels <= b.area() + POINTER, "{}", r.pixels);
    }
}

#[test]
fn a_click_repaints_where_the_window_it_raised_was_covered_and_its_button_only() {
    let (top, under) = (window_rect(W, H), under_rect(W, H));
    let covered = top.intersect(under).expect("the two windows meet");
    for (i, r) in run(Kind::Click, W, H, 6).iter().enumerate() {
        let raised = if i % 2 == 0 { under } else { top };
        let least = covered.union(button_rect(raised, H)).area();
        // The arrow's two boxes may fold in with the slack the accumulator
        // allows; never a window's whole rect.
        let most = least + 2 * POINTER;
        assert!(r.pixels >= least && r.pixels <= most, "frame {i}: {}", r.pixels);
        assert!(r.pixels < top.area(), "frame {i}");
    }
}

#[test]
fn the_harness_prints_the_capsules_lines() {
    let text = csv(&run(Kind::Caret, W, H, 3));
    let lines: Vec<_> = text.lines().collect();
    assert_eq!(lines.len(), 2 + 3);
    assert!(lines.iter().all(|l| l.starts_with("[FTRACE] ")));
    assert!(lines[2].starts_with("[FTRACE] 0,,,"), "no client stages on the host");
}
