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
//! The browser tab bar and sidebar marks: whole masks inked inside a clear
//! border, copying no other icon.

use nonos_toolkit::icons::{mask, IconId};

const SIDE: usize = 192;
const MARGIN: usize = 8;
const MARKS: [IconId; 5] = [
    IconId::Download, IconId::Layers, IconId::Globe, IconId::History, IconId::Sliders,
];

fn in_margin(i: usize) -> bool {
    let (x, y) = (i % SIDE, i / SIDE);
    x < MARGIN || y < MARGIN || x >= SIDE - MARGIN || y >= SIDE - MARGIN
}

#[test]
fn icon_nav2_icons_are_whole_square_masks() {
    for id in MARKS {
        assert_eq!(mask(id).len(), SIDE * SIDE, "{}", id.name());
    }
}

#[test]
fn icon_nav2_icons_mark_something_inside_a_clear_border() {
    for id in MARKS {
        let m = mask(id);
        assert!(m.iter().any(|&a| a == 255), "{} is empty", id.name());
        let stray = m.iter().enumerate().filter(|&(i, &a)| a != 0 && in_margin(i));
        assert_eq!(stray.count(), 0, "{} reaches its border", id.name());
    }
}

#[test]
fn icon_nav2_icons_differ_from_every_other_icon() {
    for id in MARKS {
        for other in IconId::ALL.into_iter().filter(|&o| o != id) {
            assert_ne!(mask(id), mask(other), "{} copies {}", id.name(), other.name());
        }
    }
}
