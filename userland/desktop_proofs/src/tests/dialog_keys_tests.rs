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

//! A desktop dialog answers the keyboard: it opens on the button that leaves
//! things as they are, Tab and the arrows move between its two buttons,
//! Enter presses the one with the ring, and Esc always leaves things be.

use crate::dialog_keys::{Choice, DialogFocus, KEY_ENTER, KEY_ESC, KEY_LEFT, KEY_RIGHT, KEY_TAB};

#[test]
fn a_dialog_opens_on_the_safe_button_so_a_stray_enter_changes_nothing() {
    let mut f = DialogFocus::new();
    assert_eq!(f.focused(), Choice::Leave);
    assert_eq!(f.key(KEY_ENTER), Some(Choice::Leave), "Enter on a fresh Delete keeps the file");
}

#[test]
fn tab_and_the_arrows_move_the_ring_and_enter_presses_it() {
    for mover in [KEY_TAB, KEY_LEFT, KEY_RIGHT] {
        let mut f = DialogFocus::new();
        assert_eq!(f.key(mover), None, "moving answers nothing");
        assert_eq!(f.focused(), Choice::Act);
        assert_eq!(f.key(KEY_ENTER), Some(Choice::Act));
        assert_eq!(f.key(mover), None);
        assert_eq!(f.focused(), Choice::Leave, "a second move comes back");
    }
}

#[test]
fn esc_leaves_things_be_wherever_the_ring_is() {
    let mut f = DialogFocus::new();
    assert_eq!(f.key(KEY_ESC), Some(Choice::Leave));
    f.key(KEY_TAB);
    assert_eq!(f.key(KEY_ESC), Some(Choice::Leave), "Esc never deletes or installs");
}

#[test]
fn other_keys_do_nothing_and_an_answer_puts_the_ring_back() {
    let mut f = DialogFocus::new();
    f.key(KEY_TAB);
    for code in [u32::from(b'a'), u32::from(b' '), 0x08, 0x1201] {
        assert_eq!(f.key(code), None);
        assert_eq!(f.focused(), Choice::Act, "key {code:#x} moved nothing");
    }
    f.reset();
    assert_eq!(f.focused(), Choice::Leave, "the next dialog opens on the safe button");
}

#[test]
fn the_keys_are_the_ones_the_router_sends() {
    assert_eq!(KEY_TAB, nonos_app_skeleton::KEY_TAB);
    assert_eq!(KEY_ENTER, nonos_app_skeleton::KEY_ENTER);
    assert_eq!(KEY_ESC, nonos_app_skeleton::KEY_ESC);
    assert_eq!(KEY_LEFT, nonos_app_skeleton::KEY_LEFT);
    assert_eq!(KEY_RIGHT, nonos_app_skeleton::KEY_RIGHT);
}
