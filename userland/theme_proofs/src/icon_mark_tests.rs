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
//! The shield and check icons: whole 192-pixel masks that mark something
//! inside a clear border and copy no other icon. The shield is closed and
//! hollow, inked on its rim and clear at its middle; the check is a tick
//! whose short arm falls to the left of the long one's foot.

use nonos_toolkit::icons::{mask, IconId};

const SIDE: usize = 192;
const MARGIN: usize = 8;
const MARKS: [IconId; 2] = [IconId::Shield, IconId::Check];

fn in_margin(i: usize) -> bool {
    let (x, y) = (i % SIDE, i / SIDE);
    x < MARGIN || y < MARGIN || x >= SIDE - MARGIN || y >= SIDE - MARGIN
}

#[test]
fn icon_mark_icons_are_whole_square_masks() {
    for id in MARKS {
        assert_eq!(mask(id).len(), SIDE * SIDE, "{}", id.name());
    }
}

#[test]
fn icon_mark_icons_mark_something_inside_a_clear_border() {
    for id in MARKS {
        let m = mask(id);
        assert!(m.iter().any(|&a| a == 255), "{} is empty", id.name());
        let stray = m.iter().enumerate().filter(|&(i, &a)| a != 0 && in_margin(i));
        assert_eq!(stray.count(), 0, "{} reaches its border", id.name());
    }
}

#[test]
fn icon_mark_icons_differ_from_every_other_icon() {
    for id in MARKS {
        for other in IconId::ALL.into_iter().filter(|&o| o != id) {
            assert_ne!(mask(id), mask(other), "{} copies {}", id.name(), other.name());
        }
    }
}

#[test]
fn icon_mark_shield_is_hollow_and_check_a_tick() {
    let at = |id: IconId, x: usize, y: usize| mask(id)[y * SIDE + x];
    let mid = SIDE / 2;
    assert_eq!(at(IconId::Shield, mid, mid), 0, "shield is filled at its middle");
    let row = |y: usize| (0..SIDE).filter(|&x| at(IconId::Shield, x, y) == 255).count();
    assert!(row(mid) >= 2, "shield has no sides at its middle");
    let low = |x: usize| (0..SIDE).rev().find(|&y| at(IconId::Check, x, y) == 255);
    let foot = (0..SIDE).filter_map(|x| low(x).map(|y| (y, x))).max().map(|p| p.1);
    let (left, right) = (low(SIDE / 4).unwrap_or(0), low(SIDE * 3 / 4).unwrap_or(0));
    assert!(foot.is_some_and(|x| x > SIDE / 4 && x < SIDE / 2), "check foot at {foot:?}");
    assert!(left > right, "check's short arm should sit lower than the long arm's top");
}
