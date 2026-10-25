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

//! C8v: a positioner places a popup where xdg-shell says: at its anchor,
//! grown by gravity, moved by its offset, then flipped, slid or cut to its
//! bounds, an axis at a time.

use crate::wayland_popup::place::place;
use crate::wayland_popup::positioner::{set, Positioner, FLIP_Y, RESIZE_Y, SLIDE_X};

/// A 50x40 menu below a button at (10, 10), 20x10: anchored at the button's
/// bottom left corner, growing down and to the right.
fn menu(adjust: u32) -> Positioner {
    let mut p = Positioner::default();
    for (op, w) in [(1, vec![50, 40]), (2, vec![10, 10, 20, 10]), (3, vec![6]), (4, vec![8])] {
        assert!(set(&mut p, op, &w));
    }
    assert!(set(&mut p, 5, &[adjust]));
    p
}

#[test]
fn a_menu_opens_below_its_button_and_flips_slides_or_shrinks_to_fit() {
    let roomy = (0, 0, 200, 200);
    assert_eq!(place(&menu(0), roomy), (10, 20, 50, 40));
    let mut low = menu(FLIP_Y);
    assert!(set(&mut low, 2, &[10, 170, 20, 10]));
    assert_eq!(place(&low, roomy), (10, 130, 50, 40), "flipped above the button");
    let mut right = menu(SLIDE_X);
    assert!(set(&mut right, 2, &[180, 10, 20, 10]));
    assert_eq!(place(&right, roomy), (150, 20, 50, 40), "slid back inside");
    let short = (0, 0, 200, 45);
    assert_eq!(place(&menu(FLIP_Y | RESIZE_Y), short), (10, 20, 50, 25), "neither side fits: cut");
    assert_eq!(place(&menu(0), short), (10, 20, 50, 40), "no adjustment: clipped where drawn");
}

#[test]
fn no_positioner_overflows_whatever_its_numbers() {
    let mut p = menu(0x3F);
    let edges = [i32::MIN as u32, i32::MAX as u32, 0, 1];
    for a in edges {
        for b in edges {
            let _ = set(&mut p, 2, &[a, b, i32::MAX as u32, i32::MAX as u32]);
            let _ = set(&mut p, 6, &[b, a]);
            let _ = set(&mut p, 1, &[i32::MAX as u32, 1]);
            let (_, _, w, h) = place(&p, (i32::MIN, i32::MIN, i32::MAX, i32::MAX));
            assert!(w >= 0 && h >= 0);
        }
    }
}
