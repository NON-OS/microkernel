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

//! Who draws a Linux window's frame when the client makes no decoration
//! (decoration.rs, undecorated): a client that bound
//! zxdg_decoration_manager_v1 and made no decoration for its toplevel by
//! its initial commit draws its own, as Qt's frameless windows do
//! (Telegram's, which showed two title bars framed); one that made a
//! decoration, or never bound the manager, is framed by NØNOS.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;

const DECO_MANAGER: u32 = 30;
const DECO: u32 = 31;
const KDE_MANAGER: u32 = 32;
const KDE_DECO: u32 = 33;

/// `c`'s initial commit, the configure acked, a 320 by 200 buffer shown:
/// the window's layer's size on the desktop.
fn shown(c: &mut Client) -> (u32, u32) {
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (320, 200));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    desk::with(|d| {
        let l = d.layers.first().expect("the window's layer");
        (l.width, l.height)
    })
}

fn knowing() -> Client {
    let mut c = toplevel(2);
    c.bind(40, &[("zxdg_decoration_manager_v1", 1, DECO_MANAGER)]);
    c.events();
    c
}

#[test]
fn a_client_that_knows_decorations_and_asks_for_none_draws_its_own_frame() {
    let mut c = knowing();
    assert_eq!(shown(&mut c), (320, 200), "its own pixels, no NØNOS frame around them");
    assert!(c.s.guest.scene.win.frame.own_frame);
}

#[test]
fn a_client_that_asks_for_a_decoration_is_framed() {
    let mut c = knowing();
    c.req(DECO_MANAGER, 1, &[A::U(DECO), A::U(TOPLEVEL)]);
    c.events();
    let (w, h) = shown(&mut c);
    assert!(w > 320 && h > 200, "framed: {w}x{h}");
    assert!(!c.s.guest.scene.win.frame.own_frame);
}

#[test]
fn a_client_that_never_bound_the_manager_is_framed_as_before() {
    let mut c = toplevel(2);
    let (w, h) = shown(&mut c);
    assert!(w > 320 && h > 200, "framed: {w}x{h}");
    assert!(!c.s.guest.scene.win.frame.own_frame);
}

#[test]
fn a_decoration_made_after_the_initial_commit_frames_the_window_again() {
    let mut c = knowing();
    assert_eq!(shown(&mut c), (320, 200));
    c.req(DECO_MANAGER, 1, &[A::U(DECO), A::U(TOPLEVEL)]);
    c.events();
    assert!(!c.s.guest.scene.win.frame.own_frame, "the server's frame from now on");
}

#[test]
fn a_kde_decoration_made_after_the_initial_commit_frames_the_window_again() {
    let mut c = knowing();
    c.bind(41, &[("org_kde_kwin_server_decoration_manager", 1, KDE_MANAGER)]);
    c.events();
    assert_eq!(shown(&mut c), (320, 200));
    c.req(KDE_MANAGER, 0, &[A::U(KDE_DECO), A::U(SURFACE)]);
    c.events();
    assert!(!c.s.guest.scene.win.frame.own_frame, "the server's frame from now on");
}
