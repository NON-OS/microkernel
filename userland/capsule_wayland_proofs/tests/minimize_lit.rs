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

//! A guest window minimised by its frame's minimise button forgets the
//! control the pointer lit (present/hide.rs). The pointer that pressed the
//! button is over another window or the dock once the window is gone, and
//! no motion reaches the hidden window to put the light out, so the frame
//! came back from the dock with its minimise button still lit (seen live
//! on Qwen).

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::input::{motion, post, Kind};
use nonos_capsule_wayland_proofs::linux::wayland::pump;
use nonos_toolkit::decorations::{hit_test_at, DecorationHit};

const POINTER: u32 = 14;
const BLUE: u32 = 0xFF20_40A0;

/// A shown, framed 400 by 300 window with a pointer.
fn window() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400, 300), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

#[test]
fn a_window_minimised_by_its_button_forgets_the_lit_control() {
    let mut c = window();
    let (w, h, _) = pixels(&c.s.guest).expect("shown");
    let q = c.s.guest.scene.win.frame.quarters;
    let button = (0..w)
        .flat_map(|x| (0..h.min(64)).map(move |y| (x, y)))
        .find(|&(x, y)| hit_test_at(w, h, false, x, y, q) == DecorationHit::MinimizeButton)
        .expect("a minimise button");
    let at = (button.0 as i32, button.1 as i32);
    motion(at.0, at.1);
    pump(&mut c.s.guest);
    assert_eq!(c.s.guest.scene.win.frame.hover, DecorationHit::MinimizeButton, "lit");
    post(Kind::ButtonDown, 0, 1, at, (0, 0));
    post(Kind::ButtonUp, 0, 1, at, (0, 0));
    pump(&mut c.s.guest);
    assert!(c.s.guest.scene.win.minimized, "minimised");
    assert!(c.s.guest.scene.win.out.is_none(), "its layer is off the screen");
    assert_eq!(c.s.guest.scene.win.frame.hover, DecorationHit::None, "the light is out");
    assert!(c.s.guest.scene.win.frame.chrome.is_none(), "painted afresh when shown");
}
