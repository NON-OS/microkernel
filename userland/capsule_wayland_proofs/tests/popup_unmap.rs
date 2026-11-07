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

//! A popup its client unmaps, a null buffer attached and committed on its
//! surface (popup/gone.rs unmapped): its window is closed with the window
//! manager and its layer taken off the screen, so the place it covered
//! shows and takes presses again, and the client is told popup_done. A
//! popup's first commit carries no buffer and is no unmap.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;

const POINTER: u32 = 14;
const POSITIONER: u32 = 40;
const POPUP_SURFACE: u32 = 42;
const POPUP_XDG: u32 = 43;
const POPUP: u32 = 44;
const POPUP_POOL: u32 = 50;
const POPUP_BUFFER: u32 = 51;
const POPUP_CONFIGURE: u16 = 0;
const POPUP_DONE: u16 = 1;
const RED: u32 = 0xFFFF_0000;

/// A shown 400 by 300 window and a 100 by 50 popup without a grab made on
/// it, configured at its first commit: drawn red when `drawn`.
fn with_popup(drawn: bool) -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c.req(WM_BASE, 1, &[A::U(POSITIONER)]);
    c.req(POSITIONER, 1, &[A::I(100), A::I(50)]);
    c.req(POSITIONER, 2, &[A::I(20), A::I(30), A::I(1), A::I(1)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP_SURFACE, 6, &[]);
    let evs = c.events();
    assert!(to(&evs, POPUP).iter().any(|e| e.opcode == POPUP_CONFIGURE), "configured");
    let serial = to(&evs, POPUP_XDG)[0].word(0);
    c.req(POPUP_XDG, 4, &[A::U(serial)]);
    if drawn {
        c.painted(SHM, POPUP_POOL, POPUP_BUFFER, (100, 50), RED);
        c.req(POPUP_SURFACE, 1, &[A::U(POPUP_BUFFER), A::I(0), A::I(0)]);
        c.req(POPUP_SURFACE, 2, &[A::I(0), A::I(0), A::I(100), A::I(50)]);
        c.req(POPUP_SURFACE, 6, &[]);
    }
    c.events();
    c
}

/// wl_surface.attach of no buffer, then commit, on the popup's surface.
fn unmap(c: &mut Client) -> Vec<u16> {
    c.req(POPUP_SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 6, &[]);
    to(&c.events(), POPUP).iter().map(|e| e.opcode).collect()
}

#[test]
fn a_drawn_popup_unmapped_by_its_client_leaves_the_window_manager_and_the_screen() {
    let mut c = with_popup(true);
    let (window, handle) = {
        let w = &c.s.guest.scene.popup_windows[0];
        (w.window, w.out.expect("its own surface"))
    };
    assert_eq!(unmap(&mut c), vec![POPUP_DONE]);
    assert!(c.s.guest.scene.popup_windows.is_empty());
    assert!(c.s.guest.scene.popups.list.is_empty());
    desk::with(|d| {
        assert!(d.calls.contains(&Call::Close(window)), "its window closed");
        assert!(d.calls.contains(&Call::RemoveLayer(handle)), "its layer taken off");
    });
}

#[test]
fn a_popup_unmapped_before_it_drew_leaves_the_window_manager() {
    let mut c = with_popup(false);
    let window = c.s.guest.scene.popup_windows[0].window;
    assert!(c.s.guest.scene.popup_windows[0].out.is_none(), "nothing drawn yet");
    assert_eq!(unmap(&mut c), vec![POPUP_DONE]);
    assert!(c.s.guest.scene.popup_windows.is_empty());
    desk::with(|d| assert!(d.calls.contains(&Call::Close(window))));
}

#[test]
fn a_popup_first_commit_with_no_buffer_attached_is_no_unmap() {
    let c = with_popup(false);
    assert_eq!(c.s.guest.scene.popup_windows.len(), 1);
    assert_eq!(c.s.guest.scene.popups.list.len(), 1);
    let window = c.s.guest.scene.popup_windows[0].window;
    desk::with(|d| assert!(!d.calls.contains(&Call::Close(window))));
}

#[test]
fn a_null_buffer_on_a_popup_surface_attached_before_its_first_commit_is_no_unmap() {
    let mut c = toplevel(6);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c.req(WM_BASE, 1, &[A::U(POSITIONER)]);
    c.req(POSITIONER, 1, &[A::I(100), A::I(50)]);
    c.req(POSITIONER, 2, &[A::I(20), A::I(30), A::I(1), A::I(1)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP_SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 6, &[]);
    let ops: Vec<u16> = to(&c.events(), POPUP).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![POPUP_CONFIGURE], "configured, not ended");
    assert_eq!(c.s.guest.scene.popup_windows.len(), 1);
}

#[test]
fn a_toplevel_null_buffer_still_unmaps_its_window_alone() {
    let mut c = with_popup(true);
    let parent = c.s.guest.scene.win.window.expect("a window");
    c.req(SURFACE, 1, &[A::U(0), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    let ops: Vec<u16> = to(&c.events(), POPUP).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![POPUP_DONE], "its popups end with it, as before");
    desk::with(|d| assert!(d.calls.contains(&Call::Close(parent))));
}
