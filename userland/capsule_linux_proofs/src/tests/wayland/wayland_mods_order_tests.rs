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

//! Ctrl+Q reaches a Qt client as Ctrl+Q only if the modifiers were said
//! before the Q: a client reads the state the last wl_keyboard.modifiers
//! left when a key arrives. A modifier key changes that state at once, and
//! a client is told only what it does not know.

use crate::held_keys::HeldKeys;
use crate::xkb_mods::{after, xkb_masks, CTRL};

#[test]
fn a_modifier_key_holds_or_lets_go_of_its_mask_at_once() {
    assert_eq!(after((0, 0), 29, true), Some((1 << 2, 0)), "left ctrl down");
    assert_eq!(after((1 << 2, 0), 97, false), Some((0, 0)), "right ctrl up");
    assert_eq!(after((0, 2), 42, true), Some((1, 2)), "shift keeps caps lock");
    assert_eq!(after((0, 0), 56, true), Some((1 << 3, 0)), "alt");
    assert_eq!(after((0, 0), 125, true), Some((1 << 6, 0)), "logo");
    assert_eq!(after((0, 0), 100, true), Some((1 << 7, 0)), "altgr");
}

#[test]
fn an_ordinary_key_changes_no_mask() {
    assert_eq!(after(xkb_masks(CTRL), 16, true), None, "q");
    assert_eq!(after((0, 0), 28, false), None, "enter");
}

#[test]
fn the_client_is_told_only_masks_it_does_not_have() {
    let mut held = HeldKeys::new();
    assert!(!held.mods((0, 0)), "a client starts with none held");
    assert!(held.mods(xkb_masks(CTRL)));
    assert!(!held.mods(xkb_masks(CTRL)));
    assert!(held.mods((0, 0)));
}

#[test]
fn ctrl_q_says_ctrl_before_the_q() {
    // The router's Q carries Ctrl although the client never heard Ctrl go
    // down: the masks are news, so they go before the key, not after.
    let mut held = HeldKeys::new();
    let told = xkb_masks(CTRL);
    assert_eq!(after(told, 16, true), None);
    assert!(held.mods(told), "said before the Q");
}
