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
//! press on the NØNOS frame's border, or by xdg_toplevel.resize from a press
//! on a client-drawn frame, and at the release the guest is told its new
//! size once; the buffer drawn for it is shown at the window's new corner.
//! The client sees no pointer event of the drag.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input::{motion, post, Kind};

const POINTER: u32 = 14;
const POOL2: u32 = 60;
const BUFFER2: u32 = 61;
const CONFIGURE: u16 = 0;
const POINTER_BUTTON: u16 = 3;
const POINTER_MOTION: u16 = 2;
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

/// The window's rect on screen, frame and all, as the compositor holds it.
fn on_screen() -> (u32, u32, u32, u32) {
    desk::with(|d| {
        let l = d.layers.last().expect("a layer");
        (l.x, l.y, l.width, l.height)
    })
}

/// Pressed at `from` on the window, moved to `to_`, let go: every event
/// the client was sent from the press on.
fn drag(c: &mut Client, from: (i32, i32), to_: (i32, i32)) -> Vec<Ev> {
    motion(from.0, from.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, from, (0, 0));
    motion(to_.0, to_.1);
    post(Kind::ButtonUp, 0, 1, to_, (0, 0));
    c.events()
}

/// The sizes the toplevel was told in `evs`.
fn told(evs: &[Ev]) -> Vec<(u32, u32)> {
    let configures = to(evs, TOPLEVEL).into_iter().filter(|e| e.opcode == CONFIGURE);
    configures.map(|e| (e.word(0), e.word(1))).collect()
}

/// The configure in `evs` acked and a buffer of `size` drawn for it.
fn draw(c: &mut Client, evs: &[Ev], size: (i32, i32)) {
    let serial = to(evs, XDG_SURFACE).last().expect("a configure").word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL2, BUFFER2, size, BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER2), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

fn pointer_ops(evs: &[Ev]) -> Vec<u16> {
    to(evs, POINTER).iter().map(|e| e.opcode).filter(|&op| op != 5).collect()
}

#[test]
fn the_frames_corner_dragged_resizes_the_window_at_the_release() {
    let mut c = window();
    let (x, y, w, h) = on_screen();
    let corner = (w as i32 - 2, h as i32 - 2);
    let evs = drag(&mut c, corner, (corner.0 + 50, corner.1 + 30));
    assert_eq!(told(&evs), vec![(450, 330)], "told once, at the release");
    let seen = pointer_ops(&evs);
    assert!(!seen.contains(&POINTER_BUTTON) && !seen.contains(&POINTER_MOTION), "{seen:?}");
    draw(&mut c, &evs, (450, 330));
    assert_eq!(on_screen(), (x, y, w + 50, h + 30), "its corner kept, its size the new one");
}

#[test]
fn the_left_edge_dragged_moves_the_window_with_its_new_size() {
    let mut c = window();
    let (x, y, w, h) = on_screen();
    let edge = (1, h as i32 / 2);
    let evs = drag(&mut c, edge, (edge.0 - 30, edge.1));
    assert_eq!(told(&evs), vec![(430, 300)]);
    draw(&mut c, &evs, (430, 300));
    assert_eq!(on_screen(), (x - 30, y, w + 30, h), "the right edge where it was");
}

#[test]
fn a_resize_never_leaves_the_window_smaller_than_its_least() {
    let mut c = window();
    let (_, _, w, h) = on_screen();
    let corner = (w as i32 - 2, h as i32 - 2);
    let evs = drag(&mut c, corner, (-5000, -5000));
    assert_eq!(told(&evs), vec![(64, 48)]);
}

#[test]
fn a_resize_asked_by_a_client_drawn_frame_holds_that_edge() {
    let mut c = window();
    let (x, y, w, h) = on_screen();
    let on = (w as i32 / 2, h as i32 / 2);
    motion(on.0, on.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, on, (0, 0));
    let evs = c.events();
    let press = to(&evs, POINTER).into_iter().find(|e| e.opcode == POINTER_BUTTON);
    let serial = press.expect("pressed").word(0);
    // Bottom right, 10.
    c.req(TOPLEVEL, 6, &[A::U(SEAT), A::U(serial), A::U(10)]);
    c.events();
    motion(on.0 + 20, on.1 + 10);
    post(Kind::ButtonUp, 0, 1, (on.0 + 20, on.1 + 10), (0, 0));
    let evs = c.events();
    assert_eq!(told(&evs), vec![(420, 310)]);
    assert!(
        pointer_ops(&evs).iter().all(|&op| op != POINTER_MOTION),
        "the drag is not the client's"
    );
    draw(&mut c, &evs, (420, 310));
    assert_eq!(on_screen(), (x, y, w + 20, h + 10));
}

#[test]
fn a_resize_with_a_stale_serial_holds_nothing() {
    let mut c = window();
    let (_, _, w, h) = on_screen();
    let on = (w as i32 / 2, h as i32 / 2);
    motion(on.0, on.1);
    c.events();
    c.req(TOPLEVEL, 6, &[A::U(SEAT), A::U(12345), A::U(10)]);
    c.events();
    post(Kind::ButtonDown, 0, 1, on, (0, 0));
    motion(on.0 + 20, on.1 + 10);
    post(Kind::ButtonUp, 0, 1, (on.0 + 20, on.1 + 10), (0, 0));
    assert!(told(&c.events()).is_empty());
}

/// Another button pressed while an edge is held is the frame's and ends
/// nothing: the resize goes on to the release.
#[test]
fn a_second_press_while_an_edge_is_held_does_not_drop_the_resize() {
    let mut c = window();
    let (_, _, w, h) = on_screen();
    let corner = (w as i32 - 2, h as i32 - 2);
    motion(corner.0, corner.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, corner, (0, 0));
    post(Kind::ButtonDown, 0, 2, corner, (0, 0));
    motion(corner.0 + 50, corner.1 + 30);
    post(Kind::ButtonUp, 0, 1, (corner.0 + 50, corner.1 + 30), (0, 0));
    let evs = c.events();
    assert_eq!(told(&evs), vec![(450, 330)]);
    assert!(!pointer_ops(&evs).contains(&POINTER_BUTTON), "no press reached the client");
}

/// The pointer over an edge takes the resize arrow a press there acts with,
/// over a corner the diagonal, and the arrow again on the content, so the
/// guest's own shape is not left under a resize arrow.
#[test]
fn the_pointer_over_an_edge_shows_how_a_press_there_resizes() {
    use nonos_app_skeleton::desk::Call;
    use nonos_toolkit::decorations::{ARROW_CURSOR, EW_CURSOR, NWSE_CURSOR};
    let mut c = window();
    let asked = || {
        desk::with(|d| {
            let mut codes = d.calls.iter().filter_map(|c| match c {
                Call::Cursor(_, code) => Some(*code),
                _ => None,
            });
            codes.next_back()
        })
    };
    let (_, _, w, h) = on_screen();
    motion(w as i32 - 2, h as i32 / 2);
    c.events();
    assert_eq!(asked(), Some(EW_CURSOR), "an edge's straight arrow");
    motion(w as i32 - 2, h as i32 - 2);
    c.events();
    assert_eq!(asked(), Some(NWSE_CURSOR), "a corner's diagonal");
    motion(w as i32 / 2, h as i32 / 2);
    c.events();
    assert_eq!(asked(), Some(ARROW_CURSOR), "the content's arrow back");
}

/// A client's least size bounds the drag: its left edge dragged past it
/// stops there, the configure says the least, and the buffer drawn for it
/// leaves the right edge where it was.
#[test]
fn a_resize_stops_at_the_least_size_the_client_set() {
    let mut c = window();
    c.req(TOPLEVEL, 8, &[A::I(350), A::I(280)]);
    c.events();
    let (x, y, w, h) = on_screen();
    let edge = (1, h as i32 / 2);
    let evs = drag(&mut c, edge, (edge.0 + 100, edge.1));
    assert_eq!(told(&evs), vec![(350, 300)]);
    draw(&mut c, &evs, (350, 300));
    let (nx, ny, nw, nh) = on_screen();
    assert_eq!((nx + nw, ny, nh), (x + w, y, h), "the right edge where it was");
    assert_eq!(nw, w - 50);
}

/// Where the 400-wide content's right edge is on the window: the frame's
/// side margins are the same each side.
fn content_right(w: u32) -> i32 {
    let side = (w - 400) / 2;
    (side + 400) as i32
}

/// A press just inside the right border, on the content's last columns,
/// resizes, as on a native window; live, presses 1 to 3 px inside
/// Telegram's border reached the client and nothing resized.
#[test]
fn a_press_just_inside_the_border_resizes_and_the_client_sees_none_of_it() {
    let mut c = window();
    let (_, _, w, h) = on_screen();
    let inside = (content_right(w) - 2, h as i32 / 2);
    let evs = drag(&mut c, inside, (inside.0 + 40, inside.1));
    assert_eq!(told(&evs), vec![(440, 300)], "told once, at the release");
    let seen = pointer_ops(&evs);
    assert!(!seen.contains(&POINTER_BUTTON) && !seen.contains(&POINTER_MOTION), "{seen:?}");
}

/// Past that band the content is the client's: a scroll bar along the
/// right edge still takes its press.
#[test]
fn a_press_further_in_is_the_clients_own() {
    let mut c = window();
    let (_, _, w, h) = on_screen();
    let on = (content_right(w) - 6, h as i32 / 2);
    motion(on.0, on.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, on, (0, 0));
    post(Kind::ButtonUp, 0, 1, on, (0, 0));
    let evs = c.events();
    assert!(told(&evs).is_empty(), "no resize");
    let presses = pointer_ops(&evs).iter().filter(|&&op| op == POINTER_BUTTON).count();
    assert_eq!(presses, 2, "the press and its release reached the client");
}

const REGISTRY2: u32 = 69;
const VIEWPORTER: u32 = 70;
const VIEWPORT: u32 = 71;
const SET_GEOMETRY: u16 = 3;
const SET_DESTINATION: u16 = 2;
const SHADE: i32 = 11;

/// A configure of `serial` acked and drawn at `size` inside an 11 px shadow
/// the client draws itself, its viewport's destination the whole buffer, as
/// Qt 6 draws Telegram's window.
fn draw_shadowed(c: &mut Client, serial: u32, (pool, buffer): (u32, u32), (w, h): (i32, i32)) {
    let whole = (w + 2 * SHADE, h + 2 * SHADE);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.req(XDG_SURFACE, SET_GEOMETRY, &[A::I(SHADE), A::I(SHADE), A::I(w), A::I(h)]);
    c.req(VIEWPORT, SET_DESTINATION, &[A::I(whole.0), A::I(whole.1)]);
    c.painted(SHM, pool, buffer, whole, BLUE);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

#[test]
fn a_resize_of_a_window_with_its_own_shadow_and_a_viewport_tells_its_geometrys_size() {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.bind(REGISTRY2, &[("wp_viewporter", 1, VIEWPORTER)]);
    c.req(VIEWPORTER, 1, &[A::U(VIEWPORT), A::U(SURFACE)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    draw_shadowed(&mut c, serial, (POOL, BUFFER), (400, 300));
    let (x, y, w, h) = on_screen();
    let on = (w as i32 / 2, h as i32 / 2);
    motion(on.0, on.1);
    c.events();
    post(Kind::ButtonDown, 0, 1, on, (0, 0));
    let evs = c.events();
    let press = to(&evs, POINTER).into_iter().find(|e| e.opcode == POINTER_BUTTON);
    let serial = press.expect("pressed").word(0);
    // Bottom right, 10.
    c.req(TOPLEVEL, 6, &[A::U(SEAT), A::U(serial), A::U(10)]);
    c.events();
    motion(on.0 + 20, on.1 + 10);
    post(Kind::ButtonUp, 0, 1, (on.0 + 20, on.1 + 10), (0, 0));
    let evs = c.events();
    assert_eq!(told(&evs), vec![(420, 310)], "its geometry's size, not its surface's");
    let serial = to(&evs, XDG_SURFACE).last().expect("a configure").word(0);
    draw_shadowed(&mut c, serial, (POOL2, BUFFER2), (420, 310));
    assert_eq!(on_screen(), (x, y, w + 20, h + 10), "grown by the drag alone");
}
