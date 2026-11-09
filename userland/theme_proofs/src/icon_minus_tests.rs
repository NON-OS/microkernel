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
//! The minus action icon: a whole 192-pixel mask that marks something, keeps a
//! clear border, copies no other icon, and is the plus icon's horizontal arm.

use nonos_toolkit::icons::{mask, IconId};

const SIDE: usize = 192;
const MARGIN: usize = 8;

fn in_margin(i: usize) -> bool {
    let (x, y) = (i % SIDE, i / SIDE);
    x < MARGIN || y < MARGIN || x >= SIDE - MARGIN || y >= SIDE - MARGIN
}

#[test]
fn icon_minus_is_a_whole_square_mask() {
    assert_eq!(mask(IconId::Minus).len(), SIDE * SIDE);
}

#[test]
fn icon_minus_marks_something_inside_a_clear_border() {
    let m = mask(IconId::Minus);
    assert!(m.iter().any(|&a| a == 255), "minus is empty");
    let stray = m.iter().enumerate().filter(|&(i, &a)| a != 0 && in_margin(i));
    assert_eq!(stray.count(), 0, "minus reaches its border");
}

#[test]
fn icon_minus_differs_from_every_other_icon() {
    for other in IconId::ALL.into_iter().filter(|&o| o != IconId::Minus) {
        assert_ne!(mask(IconId::Minus), mask(other), "minus copies {}", other.name());
    }
}

#[test]
fn icon_minus_is_plus_without_its_vertical_arm() {
    let (minus, plus) = (mask(IconId::Minus), mask(IconId::Plus));
    let outside = minus.iter().zip(plus).filter(|&(&m, &p)| m > p).count();
    assert_eq!(outside, 0, "minus inks where plus does not");
    let ink = |m: &[u8]| m.iter().map(|&a| a as u32).sum::<u32>();
    assert!(ink(plus) > ink(minus) * 3 / 2, "plus should carry clearly more ink");
}
