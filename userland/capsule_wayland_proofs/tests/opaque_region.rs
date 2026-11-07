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

//! A pinned window whose client calls its whole surface opaque is drawn by
//! the compositor with its alpha unread, as an XRGB buffer is; any other
//! region, or none, leaves its ARGB pixels blended over the frame's ground
//! (region.rs, region_cover.rs, present/pinned/show.rs).

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::call::set_clock;

const W: i32 = 320;
const H: i32 = 200;
const REGION: u32 = 40;
const OTHER_REGION: u32 = 41;
/// wl_compositor.create_region; wl_region.destroy, add, subtract.
const CREATE_REGION: u16 = 1;
const REGION_DESTROY: u16 = 0;
const REGION_ADD: u16 = 1;
const REGION_SUBTRACT: u16 = 2;
/// wl_surface.set_opaque_region.
const SET_OPAQUE: u16 = 4;
/// A second registry, wp_viewporter and the window surface's viewport.
const REGISTRY2: u32 = 69;
const VIEWPORTER: u32 = 70;
const VIEWPORT: u32 = 71;
/// A buffer shown on a surface four fifths its size, as foot draws at 1.25.
const DEST: (i32, i32) = (W * 4 / 5, H * 4 / 5);

/// A region's requests, add or subtract, each with its rectangle.
type Said = (u16, (i32, i32, i32, i32));

/// A framed toplevel with pins allowed, shown twice whole from buffer 101,
/// with ARGB buffers 101, 103 and 105 painted.
fn window() -> Client {
    window_on((W as u32 + 256, H as u32 + 256))
}

/// The same on a display of `size`, whose size sets its scale.
fn window_on(size: (u32, u32)) -> Client {
    nonos_libc::world::with(|w| w.allow_pins = true);
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = size);
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("opaque")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..3 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), 0xFF20_40A0 + n);
    }
    commit(&mut c, 101);
    commit(&mut c, 101);
    c
}

fn commit(c: &mut Client, buffer: u32) -> Vec<Ev> {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(0), A::I(0), A::I(W), A::I(H)]);
    c.req(SURFACE, 6, &[]);
    c.events()
}

fn region(c: &mut Client, id: u32, steps: &[Said]) {
    c.req(COMPOSITOR, CREATE_REGION, &[A::U(id)]);
    for &(op, (x, y, w, h)) in steps {
        c.req(id, op, &[A::I(x), A::I(y), A::I(w), A::I(h)]);
    }
}

/// Whether the last content the compositor took was named opaque.
fn last_opaque() -> Option<bool> {
    desk::with(|d| d.content_opaque.last().copied())
}

/// The window's next commit, from 103 after 101, once `steps` are its
/// opaque region: whether its content was named opaque.
fn opaque_after(steps: &[Said]) -> Option<bool> {
    let mut c = window();
    region(&mut c, REGION, steps);
    c.req(SURFACE, SET_OPAQUE, &[A::U(REGION)]);
    commit(&mut c, 103);
    last_opaque()
}

#[test]
fn a_region_over_the_whole_surface_names_the_pinned_content_opaque() {
    assert_eq!(opaque_after(&[(REGION_ADD, (0, 0, W, H))]), Some(true));
    assert_eq!(opaque_after(&[(REGION_ADD, (-10, -10, W + 20, H + 20))]), Some(true));
}

#[test]
fn with_no_region_an_argb_buffer_is_blended() {
    let mut c = window();
    commit(&mut c, 103);
    assert_eq!(last_opaque(), Some(false));
}

#[test]
fn a_region_with_a_hole_or_short_of_the_surface_is_not_opaque() {
    let whole = (REGION_ADD, (0, 0, W, H));
    assert_eq!(opaque_after(&[whole, (REGION_SUBTRACT, (10, 10, 4, 4))]), Some(false));
    assert_eq!(opaque_after(&[(REGION_ADD, (0, 0, W - 1, H))]), Some(false));
    assert_eq!(opaque_after(&[(REGION_ADD, (0, 1, W, H))]), Some(false));
    // Halves that together hold it are not counted: never opaque where unsure.
    let halves = [(REGION_ADD, (0, 0, W / 2, H)), (REGION_ADD, (W / 2, 0, W / 2, H))];
    assert_eq!(opaque_after(&halves), Some(false));
}

#[test]
fn a_subtract_outside_the_surface_and_an_add_after_a_hole_keep_it_opaque() {
    let whole = (REGION_ADD, (0, 0, W, H));
    assert_eq!(opaque_after(&[whole, (REGION_SUBTRACT, (W, 0, 50, H))]), Some(true));
    assert_eq!(opaque_after(&[whole, (REGION_SUBTRACT, (0, 0, 9, 9)), whole]), Some(true));
}

#[test]
fn the_region_holds_from_the_commit_and_as_it_was_set() {
    let mut c = window();
    region(&mut c, REGION, &[(REGION_ADD, (0, 0, W, H))]);
    c.req(SURFACE, SET_OPAQUE, &[A::U(REGION)]);
    // Changed and destroyed after it was set: the surface kept a copy.
    c.req(REGION, REGION_SUBTRACT, &[A::I(0), A::I(0), A::I(W), A::I(H)]);
    c.req(REGION, REGION_DESTROY, &[]);
    commit(&mut c, 103);
    assert_eq!(last_opaque(), Some(true));
    // Kept over later commits until another is set.
    commit(&mut c, 105);
    assert_eq!(last_opaque(), Some(true));
    // A null region takes it away at the next commit.
    c.req(SURFACE, SET_OPAQUE, &[A::U(0)]);
    commit(&mut c, 103);
    assert_eq!(last_opaque(), Some(false));
}

#[test]
fn a_region_set_after_a_commit_waits_for_the_next() {
    let mut c = window();
    commit(&mut c, 103);
    assert_eq!(last_opaque(), Some(false));
    region(&mut c, OTHER_REGION, &[(REGION_ADD, (0, 0, W, H))]);
    c.req(SURFACE, SET_OPAQUE, &[A::U(OTHER_REGION)]);
    assert_eq!(last_opaque(), Some(false), "not before its commit");
    commit(&mut c, 105);
    assert_eq!(last_opaque(), Some(true));
}

#[test]
fn an_opaque_region_naming_no_region_is_the_client_s_error() {
    let mut c = window();
    c.req(SURFACE, SET_OPAQUE, &[A::U(101)]);
    let evs = c.events();
    let error = to(&evs, 1).into_iter().find(|e| e.opcode == 0);
    assert!(error.is_some(), "a wl_buffer named as a region ends the client: {evs:?}");
}

/// The window's next commit from 103 with its surface given a viewport to
/// DEST and `steps` as its opaque region: whether its content was named
/// opaque, if it was shown from its pin.
fn opaque_with_viewport(steps: &[Said]) -> Option<bool> {
    // 1920x1080 is shown at 1.25 (the brand rule), so a buffer 1.25 times
    // its surface is the display's own pixels, shown at its own size.
    let mut c = window_on((1920, 1080));
    c.bind(REGISTRY2, &[("wp_viewporter", 1, VIEWPORTER)]);
    c.req(VIEWPORTER, 1, &[A::U(VIEWPORT), A::U(SURFACE)]);
    c.req(VIEWPORT, 2, &[A::I(DEST.0), A::I(DEST.1)]);
    // The new size is drawn whole first, then shown from its pins.
    commit(&mut c, 101);
    commit(&mut c, 105);
    region(&mut c, REGION, steps);
    c.req(SURFACE, SET_OPAQUE, &[A::U(REGION)]);
    let before = desk::with(|d| d.content_opaque.len());
    commit(&mut c, 103);
    desk::with(|d| d.content_opaque.get(before..).and_then(|n| n.last().copied()))
}

#[test]
fn with_a_viewport_the_region_is_held_to_its_destination() {
    // foot at a 1.25 scale: its region is in the surface's pixels, the
    // viewport's destination, not the buffer's.
    assert_eq!(opaque_with_viewport(&[(REGION_ADD, (0, 0, DEST.0, DEST.1))]), Some(true));
    assert_eq!(opaque_with_viewport(&[(REGION_ADD, (0, 0, i32::MAX, i32::MAX))]), Some(true));
    assert_eq!(opaque_with_viewport(&[(REGION_ADD, (0, 0, DEST.0 - 1, DEST.1))]), Some(false));
}
