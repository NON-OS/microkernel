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

use capsule_browser_proofs::browser::ui::select_menu::row_at;

use super::kit::{at_scale, changed, inside, list_px, open, PAGE, SIZES};
use super::spots::{row_mid, select, spots};

#[test]
fn a_heading_or_disabled_option_never_lights_or_is_chosen() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (mut list, at) = open(SIZES, select(), PAGE);
        list.hi = 3;
        let unlit = list_px(&list, at);
        list.hi = 2;
        assert_eq!(list_px(&list, at), unlit, "Gone takes no wash at {q}");
        assert!(spots(&list, at).iter().all(|(_, r)| !matches!(r, Some(2 | 3))), "at {q}");
        let (x, y) = row_mid(at, 3);
        assert!(inside(at, (x, y)) && row_at(&list, at, x, y).is_none());
        list.pointer.pressed(x, y);
        assert_eq!(list_px(&list, at), unlit, "a press on the heading holds nothing at {q}");
    }
}

#[test]
fn a_press_holds_its_row_only_while_the_pointer_stays_on_it() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (mut list, at) = open(SIZES, select(), PAGE);
        list.hi = 3;
        let unlit = list_px(&list, at);
        let (x, y) = row_mid(at, 4);
        list.pointer.pressed(x, y);
        let held = changed(&unlit, &list_px(&list, at));
        assert!(!held.is_empty(), "Large is held at {q}");
        assert!(held.iter().all(|&(x, y)| row_at(&list, at, x, y) == Some(4)), "at {q}");
        let (x, y) = row_mid(at, 5);
        list.pointer.moved(x, y);
        assert_eq!(list_px(&list, at), unlit, "moved off, nothing is held at {q}");
    }
}

#[test]
fn a_click_beside_the_list_is_off_it_and_closes_it() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (list, at) = open(SIZES, select(), PAGE);
        let (mx, my) = ((at.x + at.w / 2) as i32, (at.y + at.h / 2) as i32);
        let (l, t, r, b) = (at.x as i32 - 1, at.y as i32 - 1, (at.x + at.w) as i32, (at.y + at.h) as i32);
        for p in [(l, my), (r, my), (mx, t), (mx, b)] {
            assert!(!inside(at, p) && row_at(&list, at, p.0, p.1).is_none(), "{p:?} at {q}");
        }
    }
}
