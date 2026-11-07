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

//! A menu closes on a press outside it and its submenus; a press inside its
//! chain, or a popup that does not grab, leaves it open.

use super::rig::{menu, Wm};
use crate::focus::closed_by_press;

const APP: u32 = 30;
const OTHER: u32 = 31;

fn menus() -> Wm {
    let mut wm = Wm::new();
    wm.window(APP, 1, (100, 100, 400, 300));
    wm.window(APP, 5, (600, 100, 200, 200));
    wm.window(OTHER, 1, (900, 500, 200, 200));
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 120, 200), true);
    wm.popup(APP, 3, 2, menu((100, 10, 20, 20), 120, 100), true);
    wm.popup(APP, 4, 5, menu((0, 0, 40, 20), 60, 60), false);
    wm
}

fn closed(wm: &Wm, pressed: Option<(u32, u32)>) -> Vec<(u32, u32)> {
    let mut out = [(0u32, 0u32); 16];
    let n = closed_by_press(&wm.table, pressed, &mut out);
    let mut v = out[..n].to_vec();
    v.sort_unstable();
    v
}

#[test]
fn a_press_inside_the_menu_or_its_submenu_keeps_it() {
    let wm = menus();
    assert_eq!(closed(&wm, Some((APP, 3))), vec![]);
    assert_eq!(closed(&wm, Some((APP, 2))), vec![(APP, 3)], "the submenu is outside the menu");
}

#[test]
fn a_press_on_its_window_another_window_or_no_window_closes_it() {
    let wm = menus();
    for pressed in [Some((APP, 1)), Some((APP, 5)), Some((OTHER, 1)), None] {
        assert_eq!(closed(&wm, pressed), vec![(APP, 2), (APP, 3)], "{pressed:?}");
    }
}

#[test]
fn a_popup_that_does_not_grab_is_never_closed_by_a_press() {
    let wm = menus();
    assert!(!closed(&wm, None).contains(&(APP, 4)));
}

#[test]
fn another_owners_window_with_the_same_id_is_outside() {
    let wm = menus();
    assert!(closed(&wm, Some((OTHER, 2))).contains(&(APP, 2)));
}
