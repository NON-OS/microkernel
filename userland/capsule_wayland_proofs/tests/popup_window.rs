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

//! A popup in a window of its own (popup/window.rs): placed by the window
//! manager against its parent's window on the whole display, so a menu
//! larger than its window keeps its size; drawn on its own surface and
//! layer; reached by the pointer events the router names it in; closed by
//! the window manager's notice. Without a window manager that places
//! popups, a popup is laid on its window instead, even one that has not
//! drawn since.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::{pixels, surface_pixels};
use nonos_capsule_wayland_proofs::input;

const POINTER: u32 = 14;
const POSITIONER: u32 = 40;
const POPUP_SURFACE: u32 = 42;
const POPUP_XDG: u32 = 43;
const POPUP: u32 = 44;
const POPUP_POOL: u32 = 50;
const POPUP_BUFFER: u32 = 51;
const POPUP_CONFIGURE: u16 = 0;
const POPUP_DONE: u16 = 1;
const POINTER_ENTER: u16 = 0;
const RED: u32 = 0xFFFF_0000;

/// A shown 400 by 300 window with a pointer, and a popup of `size` made
/// on it at (20, 30), no grab, configured and drawn red: its configure.
fn with_popup(size: (i32, i32)) -> (Client, Vec<u32>) {
    with_popup_at(size, (20, 30))
}

fn with_popup_at(size: (i32, i32), anchor: (i32, i32)) -> (Client, Vec<u32>) {
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
    c.req(POSITIONER, 1, &[A::I(size.0), A::I(size.1)]);
    c.req(POSITIONER, 2, &[A::I(anchor.0), A::I(anchor.1), A::I(1), A::I(1)]);
    // Anchored at the rect's bottom right, growing down and right.
    c.req(POSITIONER, 3, &[A::U(8)]);
    c.req(POSITIONER, 4, &[A::U(8)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    c.req(POPUP_SURFACE, 6, &[]);
    let evs = c.events();
    let configure = to(&evs, POPUP).into_iter().find(|e| e.opcode == POPUP_CONFIGURE);
    let configure = configure.expect("configured");
    let told = (0..4).map(|i| configure.word(i)).collect();
    let serial = to(&evs, POPUP_XDG)[0].word(0);
    c.req(POPUP_XDG, 4, &[A::U(serial)]);
    c.painted(SHM, POPUP_POOL, POPUP_BUFFER, size, RED);
    c.req(POPUP_SURFACE, 1, &[A::U(POPUP_BUFFER), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 2, &[A::I(0), A::I(0), A::I(size.0), A::I(size.1)]);
    c.req(POPUP_SURFACE, 6, &[]);
    c.events();
    (c, told)
}

fn red(px: &[u32]) -> usize {
    px.iter().filter(|&&p| p & 0xFF_FFFF == 0xFF_0000).count()
}

#[test]
fn a_popup_larger_than_its_window_keeps_its_size_in_a_window_of_its_own() {
    // A display it fits on below and right of its anchor.
    desk::with(|d| (d.width, d.height) = (2560, 1600));
    let (c, told) = with_popup((600, 500));
    // At the anchor, one past (20, 30), and its whole size: not cut to the
    // 400 by 300 window it opens over.
    assert_eq!(told, vec![21, 31, 600, 500]);
    let popup = &c.s.guest.scene.popup_windows[0];
    let (w, h, px) = surface_pixels(popup.out.expect("its own surface")).expect("shown");
    assert_eq!((w, h), (600, 500));
    assert_eq!(red(&px), 600 * 500);
    let parent = c.s.guest.scene.win.window.expect("a window");
    desk::with(|d| {
        let open = d.calls.iter().find(|c| matches!(c, Call::PopupOpen(..)));
        assert!(matches!(open, Some(Call::PopupOpen(_, p, (_, _, 600, 500))) if *p == parent));
        assert_eq!(d.layers.len(), 2);
    });
}

#[test]
fn a_pointer_event_on_the_popup_window_reaches_the_popup() {
    let (mut c, _) = with_popup((100, 50));
    let window = c.s.guest.scene.popup_windows[0].window;
    input::on_window(Some(window));
    input::motion(10, 12);
    input::on_window(None);
    let evs = c.events();
    let enter = to(&evs, POINTER).into_iter().find(|e| e.opcode == POINTER_ENTER);
    let enter = enter.expect("entered");
    assert_eq!(enter.word(1), POPUP_SURFACE);
    assert_eq!((enter.word(2) >> 8, enter.word(3) >> 8), (10, 12), "at its own point");
}

#[test]
fn the_window_manager_closing_a_popup_tells_its_client() {
    let (mut c, _) = with_popup((100, 50));
    let (window, handle) = {
        let w = &c.s.guest.scene.popup_windows[0];
        (w.window, w.out.expect("its surface"))
    };
    desk::popup_done_notice(window);
    // The router's next event pumps the inbox.
    input::motion(5, 5);
    let evs = c.events();
    let ops: Vec<u16> = to(&evs, POPUP).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![POPUP_DONE]);
    assert!(c.s.guest.scene.popup_windows.is_empty());
    desk::with(|d| {
        assert!(d.calls.contains(&Call::RemoveLayer(handle)));
        assert!(d.calls.contains(&Call::Close(window)));
    });
}

/// No window manager placing popups: laid on its window, which has not
/// drawn since, on the pixels it last showed.
#[test]
fn without_popup_windows_a_popup_is_laid_on_its_window() {
    desk::with(|d| d.popups = false);
    let (c, told) = with_popup((100, 50));
    assert_eq!(told, vec![21, 31, 100, 50]);
    assert!(c.s.guest.scene.popup_windows.is_empty());
    let (_, _, px) = pixels(&c.s.guest).expect("shown");
    assert_eq!(red(&px), 100 * 50);
}

/// Past the display's edge, the window manager keeps it on the display.
#[test]
fn a_popup_past_the_display_is_kept_on_it() {
    let (c, told) = with_popup((600, 500));
    let (dw, dh) = desk::with(|d| (d.width, d.height));
    let w = &c.s.guest.scene.popup_windows[0];
    let (x, y) = w.at;
    assert!(x + 600 <= dw && y + 500 <= dh, "on the display: {x},{y}");
    assert_eq!((told[2], told[3]), (600, 500), "its whole size");
}

#[test]
fn an_anchor_at_the_largest_coordinate_stays_at_the_far_corner() {
    // The anchor in display pixels is the parent's corner plus the rect's,
    // which a client can set to i32::MAX: added unchecked it wrapped to the
    // display's far left, or ended the bridge where overflow traps.
    desk::with(|d| (d.width, d.height) = (2560, 1600));
    let (c, _) = with_popup_at((600, 500), (i32::MAX, i32::MAX));
    let (x, y) = c.s.guest.scene.popup_windows[0].at;
    assert_eq!((x + 600, y + 500), (2560, 1600), "at the far corner: {x},{y}");
}

/// A popup over the window, holding no grab (a tooltip, a completion
/// list), and the window pressed on its frame at the point `at` picks for
/// its width: its client is told popup_done then, and its layer goes in the
/// same step. The client and the point.
fn ends_at_press(at: fn(i32) -> (i32, i32)) -> (Client, (i32, i32)) {
    let (mut c, _) = with_popup((100, 50));
    let at = at(window_width());
    let handle = c.s.guest.scene.popup_windows[0].out.expect("its surface");
    input::motion(at.0, at.1);
    c.events();
    input::post(input::Kind::ButtonDown, 0, 1, at, (0, 0));
    let evs = c.events();
    let ops: Vec<u16> = to(&evs, POPUP).iter().map(|e| e.opcode).collect();
    assert_eq!(ops, vec![POPUP_DONE], "told at the press");
    assert!(c.s.guest.scene.popup_windows.is_empty());
    desk::with(|d| assert!(d.calls.contains(&Call::RemoveLayer(handle))));
    (c, at)
}

/// The window's width on screen, frame and all.
fn window_width() -> i32 {
    desk::with(|d| d.layers.first().expect("the window's layer").width) as i32
}

/// A press on the title bar starts a drag: the popups over the window end
/// at once, as at a minimise or a maximise. They were left to the window
/// manager's notice, a frame or more later, and drawn at their old place
/// while their window moved.
#[test]
fn a_title_bar_drag_ends_the_popups_over_the_window_at_once() {
    let (c, bar) = ends_at_press(|w| (w / 2, 20));
    assert!(c.s.guest.scene.win.frame.drag.is_some(), "the drag began");
    input::post(input::Kind::ButtonUp, 0, 1, bar, (0, 0));
}

/// A press on the frame's top border starts a resize, and ends them too.
#[test]
fn an_edge_resize_ends_the_popups_over_the_window_at_once() {
    let (c, edge) = ends_at_press(|w| (w / 2, 2));
    assert!(c.s.guest.scene.win.frame.resize.is_some(), "the resize began");
    input::post(input::Kind::ButtonUp, 0, 1, edge, (0, 0));
}
