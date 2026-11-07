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

//! A popup stacks over its window, is hit over it even where it reaches past
//! it, never takes the keyboard, and comes up only with its window.

use super::rig::{menu, Wm};
use crate::focus::{press_focus, topmost_hit_at, FocusModel};
use crate::z_order::{chain, raise};

const APP: u32 = 30;
const OTHER: u32 = 31;

fn desk() -> Wm {
    let mut wm = Wm::new();
    wm.window(APP, 1, (100, 100, 400, 300));
    wm
}

#[test]
fn a_popup_is_hit_over_its_window_and_past_its_edge() {
    let mut wm = desk();
    let placed = wm.popup(APP, 2, 1, menu((350, 280, 40, 20), 200, 150), true);
    assert_eq!((placed.rect.x, placed.rect.y), (450, 400));
    let over = topmost_hit_at(&wm.table, 460, 390, 0).expect("over the window");
    assert_eq!(over.window_id, 1, "above the popup, its window");
    let on = topmost_hit_at(&wm.table, 460, 410, 0).expect("the popup");
    assert_eq!((on.window_id, on.local_x, on.local_y), (2, 10, 10));
    let past = topmost_hit_at(&wm.table, 600, 500, 0).expect("past the window's edge");
    assert_eq!(past.window_id, 2);
}

#[test]
fn a_press_on_a_popup_takes_no_focus_and_raises_nothing() {
    let mut wm = desk();
    let mut focus = FocusModel::new();
    focus.set(APP, 1);
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 100, 100), true);
    let z = wm.z_of(APP, 2);
    let pressed = press_focus(&mut wm.table, &mut wm.z, &mut focus, APP, 2).expect("delivered");
    assert!(!pressed.focus_changed && !pressed.raised);
    assert_eq!(focus.current().map(|f| f.window_id), Some(1), "the window keeps the keys");
    assert_eq!(wm.z_of(APP, 2), z);
    assert!(!wm.table.find(APP, 2).expect("open").takes_focus());
}

#[test]
fn raising_a_window_brings_its_popups_up_over_it_in_their_order() {
    let mut wm = desk();
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 120, 200), false);
    wm.popup(APP, 3, 2, menu((100, 10, 20, 20), 120, 100), false);
    wm.window(OTHER, 1, (0, 0, 1280, 800));
    assert_eq!(topmost_hit_at(&wm.table, 110, 130, 0).map(|h| h.owner_pid), Some(OTHER));
    assert_eq!(raise(&mut wm.table, &mut wm.z, APP, 1), Some(true));
    let (w, p, s) = (wm.z_of(APP, 1), wm.z_of(APP, 2), wm.z_of(APP, 3));
    assert!(wm.z_of(OTHER, 1) < w && w < p && p < s, "window, popup, its submenu");
    let mut ids = [0u32; 8];
    let n = chain(&wm.table, APP, 1, &mut ids);
    assert_eq!(&ids[..n], &[1, 2, 3], "told to the compositor bottom first");
    // Its own popups over it do not make the window covered.
    assert_eq!(raise(&mut wm.table, &mut wm.z, APP, 1), Some(false));
}

#[test]
fn raising_a_popup_raises_its_window() {
    let mut wm = desk();
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 120, 200), false);
    wm.window(OTHER, 1, (0, 0, 1280, 800));
    assert_eq!(raise(&mut wm.table, &mut wm.z, APP, 2), Some(true));
    assert!(wm.z_of(APP, 1) > wm.z_of(OTHER, 1), "the window came up");
    assert!(wm.z_of(APP, 2) > wm.z_of(APP, 1), "with its popup over it");
}

#[test]
fn a_window_and_its_popups_close_together_topmost_first() {
    let mut wm = desk();
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 120, 200), true);
    wm.popup(APP, 3, 2, menu((100, 10, 20, 20), 120, 100), true);
    wm.popup(APP, 4, 1, menu((60, 0, 40, 20), 60, 60), false);
    wm.window(APP, 5, (600, 100, 200, 200));
    let mut out = [0u32; 8];
    let n = wm.table.remove_popups(APP, 1, false, &mut out);
    assert_eq!(&out[..n], &[3, 4, 2], "children before their parents");
    assert!(wm.table.find(APP, 1).is_some(), "the window itself is its owner's to close");
    assert!(wm.table.find(APP, 5).is_some(), "another window of the project stays");
}

#[test]
fn closing_a_popup_takes_its_submenus_and_leaves_its_window() {
    let mut wm = desk();
    wm.popup(APP, 2, 1, menu((0, 0, 40, 20), 120, 200), true);
    wm.popup(APP, 3, 2, menu((100, 10, 20, 20), 120, 100), true);
    let mut out = [0u32; 8];
    let n = wm.table.remove_popups(APP, 2, true, &mut out);
    assert_eq!(&out[..n], &[3, 2]);
    assert!(wm.table.find(APP, 1).is_some());
    let n = wm.table.remove_popups(APP, 1, true, &mut out);
    assert_eq!(n, 0, "a window that is not a popup is not taken by itself");
}
