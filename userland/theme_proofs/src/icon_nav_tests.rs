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
//! The browser toolbar's navigation marks: whole masks inked inside a clear
//! border, copying no other icon, arrows leaning as named, the star filled.

use nonos_toolkit::icons::{mask, IconId};

const SIDE: usize = 192;
const MARGIN: usize = 8;
const MARKS: [IconId; 10] = [
    IconId::Back, IconId::Forward, IconId::Reload, IconId::Home, IconId::Star,
    IconId::StarFilled, IconId::Reader, IconId::Tabs, IconId::Sidebar, IconId::Lock,
];

fn in_margin(i: usize) -> bool {
    let (x, y) = (i % SIDE, i / SIDE);
    x < MARGIN || y < MARGIN || x >= SIDE - MARGIN || y >= SIDE - MARGIN
}

fn ink(id: IconId) -> impl Iterator<Item = usize> {
    mask(id).iter().enumerate().filter(|&(_, &a)| a == 255).map(|(i, _)| i)
}

#[test]
fn icon_nav_icons_are_whole_square_masks() {
    for id in MARKS {
        assert_eq!(mask(id).len(), SIDE * SIDE, "{}", id.name());
    }
}

#[test]
fn icon_nav_icons_mark_something_inside_a_clear_border() {
    for id in MARKS {
        let m = mask(id);
        assert!(m.iter().any(|&a| a == 255), "{} is empty", id.name());
        let stray = m.iter().enumerate().filter(|&(i, &a)| a != 0 && in_margin(i));
        assert_eq!(stray.count(), 0, "{} reaches its border", id.name());
    }
}

#[test]
fn icon_nav_icons_differ_from_every_other_icon() {
    for id in MARKS {
        for other in IconId::ALL.into_iter().filter(|&o| o != id) {
            assert_ne!(mask(id), mask(other), "{} copies {}", id.name(), other.name());
        }
    }
}

#[test]
fn icon_nav_arrows_point_the_way_they_are_named() {
    let centre = |id| ink(id).map(|i| i % SIDE).sum::<usize>() / ink(id).count();
    let (back, forward) = (centre(IconId::Back), centre(IconId::Forward));
    assert!(back + 4 < SIDE / 2, "back ink centre at x={back}");
    assert!(forward > SIDE / 2 + 4, "forward ink centre at x={forward}");
}

#[test]
fn icon_nav_filled_star_carries_more_ink_than_its_outline() {
    let (outline, filled) = (ink(IconId::Star).count(), ink(IconId::StarFilled).count());
    assert!(filled > outline + outline / 5, "star {outline} vs filled {filled}");
}
