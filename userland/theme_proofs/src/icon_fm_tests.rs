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
//! The File Manager and Image Viewer marks: whole clear-bordered 192-pixel
//! masks that copy no other icon. The grid is four hollow cells, the tag is
//! punched, the trash narrows to its foot and the left chevron mirrors the right.

use nonos_toolkit::icons::{mask, IconId};

const SIDE: usize = 192;
const MARGIN: usize = 8;
const MARKS: [IconId; 4] = [IconId::Grid, IconId::Tag, IconId::Trash, IconId::ChevronLeft];

fn in_margin(i: usize) -> bool {
    let (x, y) = (i % SIDE, i / SIDE);
    x < MARGIN || y < MARGIN || x >= SIDE - MARGIN || y >= SIDE - MARGIN
}

#[test]
fn icon_fm_icons_are_whole_square_masks() {
    for id in MARKS {
        assert_eq!(mask(id).len(), SIDE * SIDE, "{}", id.name());
    }
}

#[test]
fn icon_fm_icons_mark_something_inside_a_clear_border() {
    for id in MARKS {
        let m = mask(id);
        assert!(m.iter().any(|&a| a == 255), "{} is empty", id.name());
        let stray = m.iter().enumerate().filter(|&(i, &a)| a != 0 && in_margin(i));
        assert_eq!(stray.count(), 0, "{} reaches its border", id.name());
    }
}

#[test]
fn icon_fm_icons_differ_from_every_other_icon() {
    for id in MARKS {
        for other in IconId::ALL.into_iter().filter(|&o| o != id) {
            assert_ne!(mask(id), mask(other), "{} copies {}", id.name(), other.name());
        }
    }
}

#[test]
fn icon_fm_icons_take_their_shapes() {
    let at = |id: IconId, x: usize, y: usize| mask(id)[y * SIDE + x];
    let span = |y: usize| (0..SIDE).filter(|&x| at(IconId::Trash, x, y) == 255).count();
    let q = SIDE * 23 / 80;
    let cells = [(q, q), (SIDE - q, q), (q, SIDE - q), (SIDE - q, SIDE - q), (SIDE / 2, SIDE / 2)];
    for (x, y) in cells {
        assert_eq!(at(IconId::Grid, x, y), 0, "grid filled at ({x},{y})");
    }
    let hole = SIDE * 13 / 40;
    assert_eq!(at(IconId::Tag, hole, hole), 0, "tag has no hole");
    assert_eq!(at(IconId::Tag, hole + SIDE / 16, hole), 255, "tag hole has no rim");
    assert!(span(SIDE * 11 / 40) > span(SIDE * 33 / 40), "trash lid not wider");
    let (l, r) = (mask(IconId::ChevronLeft), mask(IconId::ChevronRight));
    let off = (0..SIDE * SIDE).filter(|&i| l[i] != r[i / SIDE * SIDE + SIDE - 1 - i % SIDE]);
    assert!(off.count() < SIDE * 4, "left chevron does not mirror the right");
}
