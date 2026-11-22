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

//! The window's measures a script reads: the viewport's width and height
//! in CSS px, and the display scale in quarters, which the prelude turns
//! into `devicePixelRatio`. Anything else asked is 0.

use capsule_browser_proofs::browser::dom::Dom;

fn page() -> Dom {
    let mut d = Dom::new();
    d.viewport = (683, 384);
    d
}

#[test]
fn width_and_height_are_the_viewport() {
    let d = page();
    assert_eq!(d.window_metric(0, 5), 683);
    assert_eq!(d.window_metric(1, 5), 384);
}

#[test]
fn the_scale_is_passed_through_in_quarters() {
    let d = page();
    for q in [4, 5, 8] {
        assert_eq!(d.window_metric(2, q), q as i32);
    }
}

#[test]
fn an_unknown_measure_is_zero() {
    let d = page();
    for which in [-1, 3, 99, i32::MAX] {
        assert_eq!(d.window_metric(which, 5), 0);
    }
}
