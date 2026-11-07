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

/// A held modifier is not repeated: xkb never repeats one, and a release
/// and press of Shift between two letters would read as two Shifts.
#[test]
fn modifiers_and_locks_are_never_repeated_and_letters_are() {
    use crate::xkb_mods::is_modifier;
    for key in [29, 42, 54, 56, 58, 69, 97, 100, 125, 126] {
        assert!(is_modifier(key), "{key}");
    }
    for key in [16, 30, 57, 28, 14, 103] {
        assert!(!is_modifier(key), "{key}");
    }
}

/// What the client holds after the events `key_say` chose, as key.rs sends
/// them: each mask through HeldKeys, so only news reaches it.
fn told_after(held: &mut HeldKeys, client: &mut (u32, u32), linux: u32, press: bool, flags: u16) {
    use crate::xkb_mods::key_say;
    let news = if press { held.press(linux) } else { held.release(linux) };
    let say = key_say(linux, press, news, (xkb_masks(flags), xkb_masks(flags)));
    for masks in [say.before, say.after].into_iter().flatten() {
        if held.mods(masks) {
            *client = masks;
        }
    }
}

#[test]
fn a_modifier_held_before_the_keyboard_came_is_let_go_when_it_is_released() {
    // Ctrl went down before the window had the keyboard: the enter told it
    // nothing is held. Ctrl+V tells it Ctrl; letting Ctrl go must untell it,
    // or every wheel turn after is a zoom and every click a Ctrl+click.
    let mut held = HeldKeys::new();
    let mut client = (0, 0);
    told_after(&mut held, &mut client, 47, true, CTRL);
    assert_eq!(client, xkb_masks(CTRL), "Ctrl said before the V");
    told_after(&mut held, &mut client, 47, false, CTRL);
    for flags in [CTRL, 0] {
        let (mut h, mut c) = (HeldKeys::new(), (0, 0));
        told_after(&mut h, &mut c, 47, true, CTRL);
        told_after(&mut h, &mut c, 29, false, flags);
        assert_eq!(c, (0, 0), "Ctrl let go, the release carrying {flags:#x}");
    }
    told_after(&mut held, &mut client, 29, false, 0);
    assert_eq!(client, (0, 0));
}

#[test]
fn a_release_the_client_never_saw_sends_no_key() {
    use crate::xkb_mods::key_say;
    let say = key_say(29, false, false, (xkb_masks(CTRL), xkb_masks(CTRL)));
    assert_eq!((say.before, say.key, say.after), (None, false, Some((0, 0))));
    // An ordinary key, or a press past what HeldKeys holds: nothing at all.
    for (linux, press) in [(47, false), (47, true), (29, true)] {
        let say = key_say(linux, press, false, (xkb_masks(CTRL), xkb_masks(CTRL)));
        assert_eq!((say.before, say.key, say.after), (None, false, None), "{linux} {press}");
    }
}

#[test]
fn a_key_the_client_sees_is_said_as_before() {
    use crate::xkb_mods::key_say;
    let ctrl = xkb_masks(CTRL);
    assert_eq!(key_say(16, true, true, (ctrl, ctrl)).before, Some(ctrl), "Ctrl before the Q");
    assert_eq!(key_say(16, true, true, (ctrl, ctrl)).after, None);
    let down = key_say(29, true, true, ((0, 0), (0, 0)));
    assert_eq!((down.before, down.key, down.after), (None, true, Some(ctrl)));
}

#[test]
fn the_bridge_sends_what_key_say_chose_before_it_returns() {
    let key = include_str!("../../../../capsule_linux/src/linux/wayland/input/key.rs");
    let say = key.find("let say = key_say(").expect("key.rs asks key_say");
    let early = key[say..].find("if !say.key {").expect("a key the client never saw");
    let back = key[say + early..].find("return;").expect("returns");
    assert!(key[say + early..say + early + back].contains("say.after"), "its modifiers first");
}
