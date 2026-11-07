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

//! A menu opened from the keyboard (Alt+F, F10, the Menu key) takes its
//! grab with the serial of the key's release when that release was read
//! before the menu mapped, as GTK 3 does (popup/rules.rs). The grab is
//! granted and the menu shown; a serial older than a later press is not.

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;

const POSITIONER: u32 = 40;
const POPUP_SURFACE: u32 = 42;
const POPUP_XDG: u32 = 43;
const POPUP: u32 = 44;
const POPUP_CONFIGURE: u16 = 0;
const POPUP_DONE: u16 = 1;
const KEY: u16 = 3;

/// A shown, focused 400 by 300 window with a keyboard, its events read.
fn shown() -> Client {
    let mut c = toplevel(6);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The serials of the key events a tap of `code` sent: press, release.
fn tapped(c: &mut Client, code: u8) -> (u32, u32) {
    input::tap(code as u32, 0);
    let evs = c.events();
    let keys: Vec<u32> =
        to(&evs, KEYBOARD).iter().filter(|e| e.opcode == KEY).map(|e| e.word(0)).collect();
    assert_eq!(keys.len(), 2, "a press and a release");
    (keys[0], keys[1])
}

/// A popup on the window grabbing with `serial`, committed: the popup's
/// events.
fn grab(c: &mut Client, serial: u32) -> Vec<u16> {
    c.req(WM_BASE, 1, &[A::U(POSITIONER)]);
    c.req(POSITIONER, 1, &[A::I(100), A::I(80)]);
    c.req(POSITIONER, 2, &[A::I(10), A::I(10), A::I(1), A::I(1)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP, 1, &[A::U(SEAT), A::U(serial)]);
    c.req(POPUP_SURFACE, 6, &[]);
    to(&c.events(), POPUP).iter().map(|e| e.opcode).collect()
}

#[test]
fn a_menu_grabbing_with_the_key_s_release_is_shown() {
    let mut c = shown();
    let (_, released) = tapped(&mut c, b'f');
    assert_eq!(grab(&mut c, released), vec![POPUP_CONFIGURE], "configured, not dismissed");
    assert_eq!(c.s.guest.scene.popups.top().map(|p| p.id), Some(POPUP));
}

#[test]
fn a_release_before_a_later_press_grants_no_grab() {
    let mut c = shown();
    let (_, released) = tapped(&mut c, b'f');
    tapped(&mut c, b'g');
    assert_eq!(grab(&mut c, released), vec![POPUP_DONE], "an older serial");
    assert!(c.s.guest.scene.popups.top().is_none());
}
