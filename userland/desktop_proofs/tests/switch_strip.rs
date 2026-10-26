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
//! A line too long for its width is cut with an ellipsis on a character
//! Where the switcher's strip sits: centred, on screen, its cells inside it
//! side by side, as many as the width holds, and the run's window always in
//! the row (capsule_desktop_shell render/switch/geometry.rs).

use desktop_proofs::switch_geometry::{cell, first, fits, panel};

#[test]
fn switch_strip_holds_what_the_width_allows() {
    assert_eq!(fits(1920, 3), 3);
    assert_eq!(fits(1920, 40), 15);
    assert_eq!(fits(1280, 40), 10);
    assert_eq!(fits(100, 40), 1, "one at least, cut to the screen");
}

#[test]
fn switch_strip_keeps_the_run_in_the_row() {
    for n in 1..30 {
        let shown = fits(1280, n);
        for at in 0..n {
            let f = first(at, shown);
            assert!(f <= at && at < f + shown && f + shown <= n, "n {n} at {at}");
        }
    }
    assert_eq!(first(0, 10), 0, "from the start while it fits");
}

#[test]
fn switch_strip_is_centred_and_on_screen() {
    for (w, h) in [(1920, 1080), (1280, 800), (800, 600)] {
        let shown = fits(w, 40);
        let p = panel(w, h, shown);
        assert!(p.x + p.width <= w && p.y + p.height <= h, "{w}x{h}");
        assert!((p.x * 2 + p.width).abs_diff(w) <= 1, "{w}x{h} centred");
        assert!(p.y >= 48 && p.y < h / 2, "{w}x{h} above the middle, under the bar");
        let mut right = p.x;
        for i in 0..shown {
            let c = cell(p, i);
            assert!(c.x >= right && c.y >= p.y, "{w}x{h} cell {i} after the last");
            assert!(c.x + c.width <= p.x + p.width && c.y + c.height <= p.y + p.height);
            right = c.x + c.width;
        }
    }
}
