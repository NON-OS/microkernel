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
//! A guest window resized from its edges (toplevel/resize.rs): held by a
//! A framed window maximized and restored is told the size it had and comes
//! back in the frame it had: live, a restored Telegram came back 22 px
//! larger each way than it opened.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;

const POINTER: u32 = 14;
const POOL2: u32 = 60;
const BUFFER2: u32 = 61;
const POOL3: u32 = 62;
const BUFFER3: u32 = 63;
const CONFIGURE: u16 = 0;
const SET_MAXIMIZED: u16 = 9;
const UNSET_MAXIMIZED: u16 = 10;
const BLUE: u32 = 0xFF20_40A0;

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

fn on_screen() -> (u32, u32, u32, u32) {
    desk::with(|d| {
        let l = d.layers.last().expect("a layer");
        (l.x, l.y, l.width, l.height)
    })
}

fn told(evs: &[Ev]) -> Vec<(u32, u32)> {
    let configures = to(evs, TOPLEVEL).into_iter().filter(|e| e.opcode == CONFIGURE);
    configures.map(|e| (e.word(0), e.word(1))).collect()
}

/// The last configure in `evs` acked and a buffer of `size` drawn for it.
fn draw(c: &mut Client, evs: &[Ev], (pool, buffer): (u32, u32), size: (i32, i32)) {
    let serial = to(evs, XDG_SURFACE).last().expect("a configure").word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, pool, buffer, size, BLUE);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

#[test]
fn a_window_maximized_and_restored_is_told_its_own_size_and_framed_as_before() {
    let mut c = window();
    let before = on_screen();
    c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
    let evs = c.events();
    let (mw, mh) = *told(&evs).last().expect("told maximized");
    assert!(mw > 400 && mh > 300, "the display's size, {mw}x{mh}");
    draw(&mut c, &evs, (POOL2, BUFFER2), (mw as i32, mh as i32));
    c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
    let evs = c.events();
    assert_eq!(told(&evs).last(), Some(&(400, 300)), "the size it had, not its frame's");
    draw(&mut c, &evs, (POOL3, BUFFER3), (400, 300));
    assert_eq!(on_screen(), before, "the frame it had, where it had it");
}

const SET_GEOMETRY: u16 = 3;
const SHADE: i32 = 11;

/// The client draws its own 11 px shadow round the window and says where
/// the window is in its buffer, as Qt's does.
fn shadowed() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(XDG_SURFACE, SET_GEOMETRY, &[A::I(SHADE), A::I(SHADE), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (400 + 2 * SHADE, 300 + 2 * SHADE), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The last configure in `evs` acked and drawn at `size` inside the shadow.
fn draw_shadowed(c: &mut Client, evs: &[Ev], (pool, buffer): (u32, u32), (w, h): (i32, i32)) {
    let serial = to(evs, XDG_SURFACE).last().expect("a configure").word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.req(XDG_SURFACE, SET_GEOMETRY, &[A::I(SHADE), A::I(SHADE), A::I(w), A::I(h)]);
    c.painted(SHM, pool, buffer, (w + 2 * SHADE, h + 2 * SHADE), BLUE);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

#[test]
fn a_window_with_its_own_shadow_is_restored_to_its_geometrys_size() {
    let mut c = shadowed();
    let before = on_screen();
    c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
    let evs = c.events();
    let (mw, mh) = *told(&evs).last().expect("told maximized");
    draw_shadowed(&mut c, &evs, (POOL2, BUFFER2), (mw as i32, mh as i32));
    c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
    let evs = c.events();
    assert_eq!(told(&evs).last(), Some(&(400, 300)), "the geometry's size, not the buffer's");
    draw_shadowed(&mut c, &evs, (POOL3, BUFFER3), (400, 300));
    assert_eq!(on_screen(), before, "the frame it had, where it had it");
}

const REGISTRY2: u32 = 69;
const VIEWPORTER: u32 = 70;
const VIEWPORT: u32 = 71;
const SET_DESTINATION: u16 = 2;

/// `shadowed`, its surface given a viewport its whole buffer's size, as Qt 6
/// does on a display that offers fractional scaling: such a surface is shown
/// whole, shadow and all (present/scaled.rs).
fn shadowed_with_viewport() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.bind(REGISTRY2, &[("wp_viewporter", 1, VIEWPORTER)]);
    c.req(VIEWPORTER, 1, &[A::U(VIEWPORT), A::U(SURFACE)]);
    c.req(XDG_SURFACE, SET_GEOMETRY, &[A::I(SHADE), A::I(SHADE), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    draw_viewported(&mut c, serial, (POOL, BUFFER), (400, 300));
    c
}

/// A configure of `serial` acked and drawn at `size` inside the shadow, the
/// viewport's destination the whole buffer.
fn draw_viewported(c: &mut Client, serial: u32, (pool, buffer): (u32, u32), (w, h): (i32, i32)) {
    let whole = (w + 2 * SHADE, h + 2 * SHADE);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.req(XDG_SURFACE, SET_GEOMETRY, &[A::I(SHADE), A::I(SHADE), A::I(w), A::I(h)]);
    c.req(VIEWPORT, SET_DESTINATION, &[A::I(whole.0), A::I(whole.1)]);
    c.painted(SHM, pool, buffer, whole, BLUE);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

fn last_serial(evs: &[Ev]) -> u32 {
    to(evs, XDG_SURFACE).last().expect("a configure").word(0)
}

#[test]
fn a_window_with_its_own_shadow_and_a_viewport_is_restored_to_its_geometrys_size() {
    let mut c = shadowed_with_viewport();
    let before = on_screen();
    c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
    let evs = c.events();
    let (mw, mh) = *told(&evs).last().expect("told maximized");
    draw_viewported(&mut c, last_serial(&evs), (POOL2, BUFFER2), (mw as i32, mh as i32));
    c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
    let evs = c.events();
    assert_eq!(told(&evs).last(), Some(&(400, 300)), "the geometry's size, not the surface's");
    draw_viewported(&mut c, last_serial(&evs), (POOL3, BUFFER3), (400, 300));
    assert_eq!(on_screen(), before, "the frame it had, where it had it");
}
