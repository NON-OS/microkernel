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

//! C8v: the refusals a popup's grab is held to, each by name: no grab
//! without the latest press's serial or a key's release after it, none without a shown toplevel, none
//! but on the topmost grabbing popup; and a grab's end takes down every
//! popup above it, topmost first.

use crate::wayland_popup::stack::{Popup, Popups};

const ROOT: u32 = 1;

/// Popup `id` on surface `id + 100`, placed on `parent`, grabbing or not.
fn popup(id: u32, parent: u32, grabbed: bool) -> Popup {
    let rule = Default::default();
    Popup { id, surface: id + 100, parent, rect: (0, 0, 10, 10), grabbed, shown: grabbed, rule }
}

fn popups(list: Vec<Popup>, last_press: Option<u32>) -> Popups {
    Popups { list, positioners: Vec::new(), last_press, last_key_up: None, dismissed: false }
}

#[test]
fn a_menu_opened_from_the_keyboard_grabs_with_the_key_s_release() {
    // Alt+F: the press at 40, its release at 41 read before the menu maps,
    // and GTK names the latest key serial it was sent.
    let mut p = popups(vec![popup(5, ROOT, false)], None);
    p.pressed(40);
    p.last_key_up = Some(41);
    assert!(p.may_grab(5, 41, Some(ROOT)), "the release after the press");
    assert!(p.may_grab(5, 40, Some(ROOT)), "the press itself still");
    assert!(!p.may_grab(5, 42, Some(ROOT)), "a serial never sent");
    assert!(!p.may_grab(5, 39, Some(ROOT)), "an older one");
}

#[test]
fn a_press_after_a_release_is_the_only_serial_a_grab_may_name() {
    let mut p = popups(vec![popup(5, ROOT, false)], None);
    p.last_key_up = Some(41);
    p.pressed(42);
    assert_eq!(p.last_key_up, None);
    assert!(!p.may_grab(5, 41, Some(ROOT)), "the release before the press");
    assert!(p.may_grab(5, 42, Some(ROOT)));
}

#[test]
fn a_release_grants_nothing_once_the_grabs_ended() {
    let mut p = popups(vec![popup(5, ROOT, false)], Some(40));
    p.last_key_up = Some(41);
    p.forget_input();
    assert!(!p.may_grab(5, 40, Some(ROOT)));
    assert!(!p.may_grab(5, 41, Some(ROOT)));
}

#[test]
fn a_release_names_no_grab_on_a_popup_under_the_topmost() {
    let mut p = popups(vec![popup(5, ROOT, true), popup(6, 105, true), popup(8, 105, false)], None);
    p.last_key_up = Some(41);
    assert!(!p.may_grab(8, 41, Some(ROOT)), "on 5, under 6");
}

#[test]
fn no_grab_without_the_serial_of_the_latest_press() {
    let p = popups(vec![popup(5, ROOT, false)], Some(40));
    assert!(p.may_grab(5, 40, Some(ROOT)));
    assert!(!p.may_grab(5, 39, Some(ROOT)), "an older press");
    assert!(!p.may_grab(5, 41, Some(ROOT)), "a serial never sent");
    assert!(!popups(vec![popup(5, ROOT, false)], None).may_grab(5, 40, Some(ROOT)), "no press");
}

#[test]
fn no_grab_without_a_shown_toplevel_or_twice() {
    let p = popups(vec![popup(5, ROOT, false), popup(6, ROOT, true)], Some(40));
    assert!(!p.may_grab(5, 40, None), "no window shown");
    assert!(!p.may_grab(6, 40, Some(ROOT)), "6 holds its grab already");
    assert!(!p.may_grab(9, 40, Some(ROOT)), "no such popup");
}

#[test]
fn a_grab_stacks_only_on_the_topmost_grabbing_popup() {
    let open = vec![popup(5, ROOT, true), popup(6, 105, true)];
    let mut p = popups(open, Some(40));
    p.list.push(popup(7, 106, false));
    p.list.push(popup(8, 105, false));
    p.list.push(popup(9, ROOT, false));
    assert_eq!(p.top().map(|t| t.id), Some(6));
    assert!(p.may_grab(7, 40, Some(ROOT)), "on 6, the topmost");
    assert!(!p.may_grab(8, 40, Some(ROOT)), "on 5, under 6");
    assert!(!p.may_grab(9, 40, Some(ROOT)), "on the toplevel, with popups open");
    let none = popups(vec![popup(5, 105, false)], Some(40));
    assert!(!none.may_grab(5, 40, Some(ROOT)), "with none open, only on the toplevel");
}

#[test]
fn a_grab_s_end_takes_down_every_popup_above_it_topmost_first() {
    let mut p = popups(vec![popup(5, ROOT, true), popup(6, 105, true), popup(7, 106, true)], None);
    p.list.push(popup(8, ROOT, false));
    assert_eq!(p.ending(6), [7, 6]);
    assert_eq!(p.ending(5), [7, 6, 5]);
    assert_eq!(p.ending(8), [8], "a popup with no grab ends alone");
    assert_eq!(p.all(), [7, 6, 5, 8], "every one, the grabbing ones topmost first");
}

#[test]
fn no_grab_on_a_popup_already_shown() {
    let mut shown = popup(5, ROOT, false);
    shown.shown = true;
    let p = popups(vec![shown], Some(40));
    assert!(!p.may_grab(5, 40, Some(ROOT)), "mapped without a grab: it would open twice");
}

#[test]
fn a_popup_with_no_grab_ends_with_every_popup_placed_on_it() {
    // 5 on the toplevel, 6 on 5, 7 on 6, 9 beside: none grabbing.
    let list = vec![popup(5, ROOT, false), popup(6, 105, false), popup(7, 106, false), popup(9, ROOT, false)];
    let p = popups(list, None);
    assert_eq!(p.ending(6), [7, 6], "its child first");
    assert_eq!(p.ending(5), [7, 6, 5]);
    assert_eq!(p.ending(9), [9], "a sibling alone");
}
