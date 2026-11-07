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

//! A popup that ends moves the keyboard only while its window has focus.
//! A tooltip over a window in the background ended by a resize or a new
//! buffer once told the client wl_keyboard.enter, and nothing told it the
//! leave, so it thought it had the keyboard while keys went elsewhere.

const ENDS: &str = include_str!("../../../../capsule_linux/src/linux/wayland/popup/ends.rs");
const FOCUS: &str = include_str!("../../../../capsule_linux/src/linux/wayland/input/focus.rs");

fn body<'a>(src: &'a str, head: &str) -> &'a str {
    let at = src.find(head).expect(head);
    &src[at..at + src[at..].find("\n}\n").expect("end")]
}

#[test]
fn the_keyboard_follows_an_ended_popup_only_with_focus() {
    let finish = body(ENDS, "pub fn finish(");
    let gate = finish.find("match guest.scene.win.focused {").expect("asks for focus");
    let on = finish.find("keyboard_on(guest)").expect("moves the keyboard");
    assert!(gate < on, "the keyboard moves only under the focus check");
    assert_eq!(finish.matches("keyboard_on(guest)").count(), 1);
    assert!(finish[gate..].contains("false => popups_gone(guest, &surfaces),"));
    assert!(finish.contains("surfaces.push(p.surface);"), "each ended popup's surface");
}

#[test]
fn without_focus_only_a_popup_the_keyboard_was_on_is_left() {
    let gone = body(FOCUS, "pub fn popups_gone(");
    assert!(gone.contains(".filter(|s| surfaces.contains(s))"));
    assert!(gone.contains("leave_surface(guest, on);"));
    assert!(!gone.contains("keyboard_on("), "nothing enters");
    let leave = body(FOCUS, "fn leave_surface(");
    assert!(leave.contains("guest.scene.keyboard_on = None;"));
    assert!(leave.contains("HeldKeys::new()"), "every key let go with it");
    assert!(leave.contains("KEYBOARD_LEAVE"));
}
