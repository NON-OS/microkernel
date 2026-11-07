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

//! A window whose client bound no wl_seat still answers the pointer on its
//! NØNOS frame, which is the bridge's own: dragged by its title bar it
//! moves, and its maximise control takes the screen. Before, the bridge
//! took input only once the client had a pointer or keyboard, and such a
//! window's frame was dead to every press.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input::{motion, post, Kind};

const BLUE: u32 = 0xFF20_40A0;

/// A shown, framed 400 by 300 window from a client with no seat.
fn window() -> Client {
    let mut c = Client::new();
    c.bind(
        REGISTRY,
        &[("wl_compositor", 6, COMPOSITOR), ("xdg_wm_base", 6, WM_BASE), ("wl_shm", 1, SHM)],
    );
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.events();
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400, 300), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    assert!(c.s.guest.scene.pointer.is_none() && c.s.guest.scene.keyboard.is_none());
    c
}

fn on_screen() -> (u32, u32, u32, u32) {
    desk::with(|d| {
        let l = d.layers.last().expect("a layer");
        (l.x, l.y, l.width, l.height)
    })
}

#[test]
fn a_seatless_window_subscribes_for_input_once_it_is_shown() {
    let mut c = window();
    motion(10, 10);
    c.events();
    let asked = desk::with(|d| d.calls.iter().any(|k| matches!(k, Call::Subscribe(_))));
    assert!(asked, "the bridge asked the router for input");
}

#[test]
fn a_seatless_window_dragged_by_its_title_bar_moves() {
    let mut c = window();
    let (x, y, w, _) = on_screen();
    let bar = (w as i32 / 2, 20);
    motion(bar.0, bar.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, bar, (0, 0));
    c.events();
    assert!(c.s.guest.scene.win.frame.drag.is_some(), "the drag began");
    motion(bar.0 + 40, bar.1 + 24);
    c.events();
    post(Kind::ButtonUp, 0, 1, (bar.0 + 40, bar.1 + 24), (0, 0));
    c.events();
    let (nx, ny, _, _) = on_screen();
    assert_eq!((nx, ny), (x + 40, y + 24), "moved by the drag");
}
