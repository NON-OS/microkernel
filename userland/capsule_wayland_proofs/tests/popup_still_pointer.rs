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

//! A press on a popup that mapped under a pointer that has not moved since:
//! the client reads a button as on the surface it last entered, so the press
//! is told after an enter of the popup, in a frame of its own, both for a
//! popup in a window of its own and for one laid on its window. A press on
//! the surface entered sends the button alone.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;

const POINTER: u32 = 14;
const POSITIONER: u32 = 40;
const POPUP_SURFACE: u32 = 42;
const POPUP_XDG: u32 = 43;
const POPUP: u32 = 44;
const POPUP_POOL: u32 = 50;
const POPUP_BUFFER: u32 = 51;
const POPUP_CONFIGURE: u16 = 0;
const ENTER: u16 = 0;
const LEAVE: u16 = 1;
const MOTION: u16 = 2;
const BUTTON: u16 = 3;
const FRAME: u16 = 5;
const LEFT: u32 = 1;

/// A shown 400 by 300 window with a pointer, the pointer resting on it at
/// window point `at`: the client and where on its surface the pointer is.
fn resting(at: (i32, i32)) -> (Client, (i32, i32)) {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let window = c.s.guest.scene.win.window.expect("a window");
    input::on_window(Some(window));
    input::motion(at.0, at.1);
    let evs = c.events();
    let enter = to(&evs, POINTER).into_iter().find(|e| e.opcode == ENTER).expect("entered");
    assert_eq!(enter.word(1), SURFACE);
    let on = ((enter.word(2) >> 8) as i32, (enter.word(3) >> 8) as i32);
    (c, on)
}

/// A 100 by 50 popup, no grab, at surface point (21, 31), configured and
/// drawn: mapped with no pointer event since.
fn map_popup(c: &mut Client) {
    c.req(WM_BASE, 1, &[A::U(POSITIONER)]);
    c.req(POSITIONER, 1, &[A::I(100), A::I(50)]);
    c.req(POSITIONER, 2, &[A::I(20), A::I(30), A::I(1), A::I(1)]);
    c.req(POSITIONER, 3, &[A::U(8)]);
    c.req(POSITIONER, 4, &[A::U(8)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP_SURFACE, 6, &[]);
    let evs = c.events();
    assert!(to(&evs, POPUP).iter().any(|e| e.opcode == POPUP_CONFIGURE));
    let serial = to(&evs, POPUP_XDG)[0].word(0);
    c.req(POPUP_XDG, 4, &[A::U(serial)]);
    c.painted(SHM, POPUP_POOL, POPUP_BUFFER, (100, 50), 0xFFFF_0000);
    c.req(POPUP_SURFACE, 1, &[A::U(POPUP_BUFFER), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 2, &[A::I(0), A::I(0), A::I(100), A::I(50)]);
    c.req(POPUP_SURFACE, 6, &[]);
    c.events();
}

/// The pointer events of a press at window point `at` of `window`, as
/// (opcode, the surface an enter or leave names).
fn press(c: &mut Client, window: u32, at: (i32, i32)) -> Vec<(u16, u32)> {
    input::on_window(Some(window));
    input::post(input::Kind::ButtonDown, 0, LEFT, at, (0, 0));
    input::on_window(None);
    let evs = c.events();
    to(&evs, POINTER)
        .into_iter()
        .map(|e| (e.opcode, if matches!(e.opcode, ENTER | LEAVE) { e.word(1) } else { 0 }))
        .collect()
}

#[test]
fn a_press_on_a_laid_popup_mapped_under_a_still_pointer_enters_it_first() {
    desk::with(|d| d.popups = false);
    let (mut c, on) = resting((60, 70));
    // The window point over surface point (50, 50), inside the popup.
    let at = (60 + 50 - on.0, 70 + 50 - on.1);
    let window = c.s.guest.scene.win.window.expect("a window");
    input::on_window(Some(window));
    input::motion(at.0, at.1);
    c.events();
    map_popup(&mut c);
    assert!(c.s.guest.scene.popup_windows.is_empty(), "laid on its window");
    let got = press(&mut c, window, at);
    assert_eq!(
        got,
        vec![(LEAVE, SURFACE), (ENTER, POPUP_SURFACE), (FRAME, 0), (BUTTON, 0), (FRAME, 0)]
    );
}

#[test]
fn a_press_on_a_popup_window_mapped_under_a_still_pointer_enters_it_first() {
    let (mut c, _) = resting((60, 70));
    map_popup(&mut c);
    let popup = c.s.guest.scene.popup_windows[0].window;
    let got = press(&mut c, popup, (10, 12));
    assert_eq!(
        got,
        vec![(LEAVE, SURFACE), (ENTER, POPUP_SURFACE), (FRAME, 0), (BUTTON, 0), (FRAME, 0)]
    );
}

#[test]
fn a_press_on_the_surface_entered_sends_the_button_alone() {
    let (mut c, _) = resting((60, 70));
    let window = c.s.guest.scene.win.window.expect("a window");
    let got = press(&mut c, window, (60, 70));
    assert_eq!(got, vec![(BUTTON, 0), (FRAME, 0)]);
    assert!(!got.iter().any(|(op, _)| *op == MOTION));
}
