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

//! A popup that draws its own shadow round its menu and says where the
//! menu is (xdg_surface.set_window_geometry), as a GTK menu does: the
//! positioner places that box, so the box is what shows at its place, in
//! a window of its own or laid on its window, and the pointer is told
//! where it is on the surface from the box's corner (popup/corner.rs).

use nonos_app_skeleton::desk;
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
const REGISTRY2: u32 = 69;
const VIEWPORTER: u32 = 70;
const VIEWPORT: u32 = 71;
const POPUP_CONFIGURE: u16 = 0;
const POINTER_ENTER: u16 = 0;
const RED: u32 = 0xFFFF_0000;
const SHADOW: u32 = 0x8010_1010;
const MENU: (i32, i32) = (100, 50);
/// The shadow's width on every side of the menu.
const M: i32 = 12;

/// The popup's buffer: the menu red, the shadow round it when `m` is not 0.
fn attach(c: &mut Client, m: i32) {
    let (bw, bh) = (MENU.0 + 2 * m, MENU.1 + 2 * m);
    let inside = |x: i32, y: i32| x >= m && y >= m && x < m + MENU.0 && y < m + MENU.1;
    let px: Vec<u8> = (0..bh)
        .flat_map(|y| (0..bw).map(move |x| (x, y)))
        .flat_map(|(x, y)| if inside(x, y) { RED } else { SHADOW }.to_le_bytes())
        .collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"shadowed popup".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, px.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, px.as_ptr().cast(), px.len()), px.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(POPUP_POOL), A::I(px.len() as i32)], &[fd]);
    let fmt = [A::U(POPUP_BUFFER), A::I(0), A::I(bw), A::I(bh), A::I(bw * 4), A::U(0)];
    c.req(POPUP_POOL, 0, &fmt);
    c.req(POPUP_SURFACE, 1, &[A::U(POPUP_BUFFER), A::I(0), A::I(0)]);
    c.req(POPUP_SURFACE, 2, &[A::I(0), A::I(0), A::I(bw), A::I(bh)]);
}

/// A shown 400 by 300 window with a pointer, and the menu made on it at
/// (20, 30), drawn with a shadow `m` wide and its geometry set inside it:
/// the configure's place and size.
fn with_menu(m: i32) -> (Client, Vec<u32>) {
    with_menu_on(m, false)
}

/// The same, its surface given a viewport its menu's size when `viewport`.
fn with_menu_on(m: i32, viewport: bool) -> (Client, Vec<u32>) {
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
    c.req(POSITIONER, 1, &[A::I(MENU.0), A::I(MENU.1)]);
    c.req(POSITIONER, 2, &[A::I(20), A::I(30), A::I(1), A::I(1)]);
    c.req(POSITIONER, 3, &[A::U(8)]);
    c.req(POSITIONER, 4, &[A::U(8)]);
    c.req(COMPOSITOR, 0, &[A::U(POPUP_SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(POPUP_XDG), A::U(POPUP_SURFACE)]);
    c.req(POPUP_XDG, 2, &[A::U(POPUP), A::U(XDG_SURFACE), A::U(POSITIONER)]);
    if m != 0 {
        c.req(POPUP_XDG, 3, &[A::I(m), A::I(m), A::I(MENU.0), A::I(MENU.1)]);
    }
    if viewport {
        c.bind(REGISTRY2, &[("wp_viewporter", 1, VIEWPORTER)]);
        c.req(VIEWPORTER, 1, &[A::U(VIEWPORT), A::U(POPUP_SURFACE)]);
        c.req(VIEWPORT, 2, &[A::I(MENU.0 + 2 * m), A::I(MENU.1 + 2 * m)]);
    }
    c.req(POPUP_SURFACE, 6, &[]);
    let evs = c.events();
    let configure = to(&evs, POPUP).into_iter().find(|e| e.opcode == POPUP_CONFIGURE);
    let told = (0..4).map(|i| configure.expect("configured").word(i)).collect();
    let serial = to(&evs, POPUP_XDG)[0].word(0);
    c.req(POPUP_XDG, 4, &[A::U(serial)]);
    attach(&mut c, m);
    c.req(POPUP_SURFACE, 6, &[]);
    c.events();
    (c, told)
}

fn red(px: &[u32]) -> usize {
    px.iter().filter(|&&p| p == RED).count()
}

#[test]
fn a_shadowed_menu_in_a_window_of_its_own_shows_its_geometry_alone() {
    let (c, told) = with_menu(M);
    assert_eq!(told, vec![21, 31, 100, 50], "placed and sized by its geometry");
    let popup = &c.s.guest.scene.popup_windows[0];
    let (w, h, px) = surface_pixels(popup.out.expect("its own surface")).expect("shown");
    assert_eq!((w, h), (100, 50), "the window the window manager placed, no larger");
    assert_eq!(red(&px), 100 * 50, "every pixel the menu, none of the shadow");
}

#[test]
fn the_pointer_on_a_shadowed_menu_is_told_from_its_geometrys_corner() {
    let (mut c, _) = with_menu(M);
    let window = c.s.guest.scene.popup_windows[0].window;
    input::on_window(Some(window));
    input::motion(10, 12);
    input::on_window(None);
    let evs = c.events();
    let enter = to(&evs, POINTER).into_iter().find(|e| e.opcode == POINTER_ENTER);
    let enter = enter.expect("entered");
    assert_eq!(enter.word(1), POPUP_SURFACE);
    assert_eq!((enter.word(2) >> 8, enter.word(3) >> 8), (10 + 12, 12 + 12));
}

/// Laid on its window, the menu's box sits where an unshadowed menu's
/// does: at its place, the shadow round it.
#[test]
fn a_shadowed_menu_laid_on_its_window_sits_at_its_place() {
    let first_red = |m: i32| {
        desk::with(|d| d.popups = false);
        let (c, _) = with_menu(m);
        assert!(c.s.guest.scene.popup_windows.is_empty());
        let (w, _, px) = pixels(&c.s.guest).expect("shown");
        let at = px.iter().position(|&p| p == RED).expect("the menu shown");
        assert_eq!(red(&px), 100 * 50);
        (at % w as usize, at / w as usize)
    };
    assert_eq!(first_red(M), first_red(0));
}

/// A menu with a viewport is shown whole from its surface's corner, no box
/// of it cut (present/scaled.rs): the pointer on its window is told its own
/// point on the surface, with no geometry corner added.
#[test]
fn the_pointer_on_a_menu_with_a_viewport_is_told_its_own_point() {
    let (mut c, _) = with_menu_on(M, true);
    let window = c.s.guest.scene.popup_windows[0].window;
    input::on_window(Some(window));
    input::motion(10, 12);
    input::on_window(None);
    let evs = c.events();
    let enter = to(&evs, POINTER).into_iter().find(|e| e.opcode == POINTER_ENTER);
    let enter = enter.expect("entered");
    assert_eq!(enter.word(1), POPUP_SURFACE);
    assert_eq!((enter.word(2) >> 8, enter.word(3) >> 8), (10, 12));
}
