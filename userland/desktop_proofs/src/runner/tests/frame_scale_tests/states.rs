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
//! A window's frame at a scale: its measures, its pixels and its title's rows.
//! The frame says which window has the keyboard, and its title stays in the
//! bar: left after its inset, cut before the controls.

use super::rig::{paint, paint_in, px, title_ink, AT_REST, H, SCALES, W};
use nonos_toolkit::decorations::{controls_left_at, focus_rule, frame_rect_at, FrameState};

#[test]
fn the_focused_window_has_the_rule_and_the_other_does_not() {
    for q in SCALES {
        let f = frame_rect_at(W, H, false, q);
        let focused = paint(q, b"");
        let other = paint_in(q, b"", FrameState { focused: false, ..AT_REST });
        let rule = focus_rule();
        assert_eq!(focused[(f.y * W + W / 2) as usize], rule, "the rule along the top at {q}");
        assert_ne!(other[(f.y * W + W / 2) as usize], rule, "no rule unfocused at {q}");
    }
}

#[test]
fn the_title_is_left_in_its_bar_and_cut_before_the_controls() {
    let ((_, _), (t1, b1)) = title_ink(4, b"Files");
    let ((_, _), (t2, b2)) = title_ink(8, b"Files");
    let (h1, h2) = (b1 - t1 + 1, b2 - t2 + 1);
    assert!(h2 * 10 >= h1 * 17 && h2 * 10 <= h1 * 23, "type at 2x is about twice: {h1} then {h2}");
    for q in SCALES {
        let ((left, _), _) = title_ink(q, b"Files");
        let f = frame_rect_at(W, H, false, q);
        assert!(left >= f.x + px(16, q) && left < f.x + px(20, q), "left after 16 at {q}");
        let long = "A window title far longer than any bar it could be drawn in ".repeat(3);
        let ((_, right), _) = title_ink(q, long.as_bytes());
        assert!(right < controls_left_at(W, H, false, q), "cut before the controls at {q}");
    }
}
