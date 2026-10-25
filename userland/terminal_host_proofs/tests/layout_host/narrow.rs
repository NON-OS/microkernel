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

//! Behaviour as the window narrows or degenerates.

use crate::layout::limits::{LEFT_RAIL_W, MIN_BODY_W};
use crate::support::{lay, BOTH_RAILS};

/// The body has to stay a usable terminal at the shipping 1280x720 guest.
#[test]
fn the_body_stays_wide_at_the_guest_resolution() {
    let l = lay(1280, 720, BOTH_RAILS);
    assert_eq!(l.body.w, 1280 - LEFT_RAIL_W);
    assert!(l.body.w >= 900, "body squeezed to {}", l.body.w);
}

#[test]
fn the_rail_drops_on_a_narrow_window() {
    let l = lay(520, 300, BOTH_RAILS);
    assert_eq!(l.left_rail.w, 0);
    assert_eq!(l.body.w, 520);
}

/// Whatever the rails ask for, the terminal itself must keep a usable column;
/// a body squeezed to nothing is a window with no output in it.
#[test]
fn the_body_never_starves() {
    for w in [320u32, 520, 640, 700, 900, 1024, 1440, 2560] {
        let l = lay(w, 900, BOTH_RAILS);
        assert!(l.body.w >= MIN_BODY_W.min(w), "body starved at {}: got {}", w, l.body.w);
    }
}

/// The real payoff of a host harness: these sizes arise from a drag or a
/// first frame before the compositor reports a size, and never appear in a
/// screendump. Any subtraction that underflows here panics in debug and wraps
/// to a multi-gigabyte rect in release.
#[test]
fn degenerate_windows_do_not_underflow() {
    for (w, h) in [(0u32, 0u32), (1, 1), (10, 10), (100, 40), (320, 60)] {
        let l = lay(w, h, BOTH_RAILS);
        for (name, r) in
            [("left_rail", l.left_rail), ("body", l.body), ("input", l.input), ("footer", l.footer)]
        {
            assert!(r.w <= w, "{} wider than {}x{}: {:?}", name, w, h, r);
            assert!(r.h <= h, "{} taller than {}x{}: {:?}", name, w, h, r);
        }
    }
}
