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

//! The lid keys in the shell: each is a system key, closing asks for the
//! screen dark under the one choice there is, and every compositor hears
//! what the lid asks exactly once, a restarted one again.

use nonos_app_skeleton::{KEY_LID_CLOSED, KEY_LID_OPENED};

use crate::router_shell_keys::{KEY_LID_CLOSED as ROUTER_CLOSED, KEY_LID_OPENED as ROUTER_OPENED};
use crate::shell_system_keys::lid::{dark, LidTold, OnClose, ON_CLOSE};
use crate::shell_system_keys::system_key::{system_key, SystemKey};

#[test]
fn the_lid_keys_are_system_keys_with_the_routers_codes() {
    assert_eq!((KEY_LID_CLOSED, KEY_LID_OPENED), (ROUTER_CLOSED, ROUTER_OPENED));
    assert_eq!(system_key(KEY_LID_CLOSED), Some(SystemKey::Lid(true)));
    assert_eq!(system_key(KEY_LID_OPENED), Some(SystemKey::Lid(false)));
}

#[test]
fn closing_blanks_and_nothing_more_until_the_owner_chooses() {
    assert_eq!(ON_CLOSE, OnClose::Blank);
    assert!(dark(true, ON_CLOSE));
    assert!(!dark(false, ON_CLOSE));
}

#[test]
fn a_machine_with_no_lid_is_never_told_one() {
    assert_eq!(LidTold::default().due(9), None);
}

#[test]
fn each_compositor_hears_the_lid_once() {
    let mut lid = LidTold::default();
    assert!(lid.heard(true, ON_CLOSE));
    assert_eq!(lid.due(9), Some(true));
    // No answer: asked again on the next tick.
    assert_eq!(lid.due(9), Some(true));
    lid.told(true, 9);
    assert_eq!(lid.due(9), None);
    // The first reading posted again, or a repeat: nothing new to say.
    lid.heard(true, ON_CLOSE);
    assert_eq!(lid.due(9), None);
    // A restarted compositor came up lit; it is told the lid is shut.
    assert_eq!(lid.due(12), Some(true));
    lid.told(true, 12);
    assert!(!lid.heard(false, ON_CLOSE));
    assert_eq!(lid.due(12), Some(false));
    lid.told(false, 12);
    assert_eq!(lid.due(12), None);
}
