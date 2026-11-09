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

//! A client that draws its own shadow round its window, says where the
//! window is, and gives its surface a viewport, as Qt 6 does once it is
//! told a fractional scale: Telegram on a 1920x1080 display, shown at 1.25,
//! draws an 1065x815 buffer for an 852x652 surface whose window is the box
//! at 26,23 of 800x600. The window shows that box alone, at 1000x750
//! (present/crop.rs crop_dest); its shadow was shown, flattened over the
//! frame's ground, as an opaque band, and a resize told it the shadowed
//! size, which it grew by its shadow again.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::input;

/// The surface's size, the viewport's destination, and its buffer at 1.25.
const DEST: (i32, i32) = (852, 652);
const BW: i32 = 1065;
const BH: i32 = 815;
/// The window, in the surface's pixels, and as the buffer's box of it.
const G: (i32, i32, i32, i32) = (26, 23, 800, 600);
const BOX: (i32, i32, i32, i32) = (33, 29, 1000, 750);
const SHADOW: u32 = 0x6010_1010;
const BLUE: u32 = 0xFF20_40A0;
const GREEN: u32 = 0xFF20_A040;
const POINTER: u32 = 40;
const POINTER_ENTER: u16 = 0;
const POINTER_MOTION: u16 = 2;
const REGISTRY2: u32 = 69;
const VIEWPORTER: u32 = 70;
const VIEWPORT: u32 = 71;

fn within(x: i32, y: i32, r: (i32, i32, i32, i32)) -> bool {
    x >= r.0 && y >= r.1 && x < r.0 + r.2 && y < r.1 + r.3
}

/// The buffer: the shadow outside the window's box, `inside` within, and
/// the box `b` (buffer pixels) of `mark`.
fn picture(inside: u32, b: Option<((i32, i32, i32, i32), u32)>) -> Vec<u32> {
    (0..BH)
        .flat_map(|y| (0..BW).map(move |x| (x, y)))
        .map(|(x, y)| match b {
            Some((r, mark)) if within(x, y, r) => mark,
            _ if within(x, y, BOX) => inside,
            _ => SHADOW,
        })
        .collect()
}

fn attach(c: &mut Client, n: u32, px: &[u32]) {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    let bytes: Vec<u8> = px.iter().flat_map(|p| p.to_le_bytes()).collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"viewported".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(BW), A::I(BH), A::I(BW * 4), A::U(0)]);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
}

/// The window on a 1920x1080 display with its viewport to DEST and its
/// geometry `g`, its first buffer committed.
fn viewported(g: (i32, i32, i32, i32)) -> Client {
    desk::with(|d| (d.width, d.height) = (1920, 1080));
    let mut c = toplevel(6);
    c.bind(REGISTRY2, &[("wp_viewporter", 1, VIEWPORTER)]);
    c.req(VIEWPORTER, 1, &[A::U(VIEWPORT), A::U(SURFACE)]);
    c.req(VIEWPORT, 2, &[A::I(DEST.0), A::I(DEST.1)]);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(XDG_SURFACE, 3, &[A::I(g.0), A::I(g.1), A::I(g.2), A::I(g.3)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    attach(&mut c, 0, &picture(BLUE, None));
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The window's corner of content, and the BOX.2 by BOX.3 pixels shown there.
fn content(c: &Client) -> ((usize, usize), Vec<u32>) {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let w = w as usize;
    let at = px.iter().position(|&p| p == BLUE || p == GREEN).expect("content shown");
    let (x0, y0) = (at % w, at / w);
    let rows = (0..BOX.3 as usize).flat_map(|y| (0..BOX.2 as usize).map(move |x| (y, x)));
    ((x0, y0), rows.map(|(y, x)| px[(y0 + y) * w + x0 + x]).collect())
}

#[test]
fn the_window_is_its_geometry_in_the_viewports_pixels_and_the_shadow_is_not_shown() {
    let c = viewported(G);
    let (w, h, stride) = c.s.guest.scene.win.shape.expect("shaped");
    assert_eq!((w, h, stride), (BOX.2 as u32, BOX.3 as u32, BW as u32 * 4), "the box, at 1.25");
    assert_eq!(c.s.guest.scene.win.origin, (G.0, G.1), "the geometry's corner, in surface pixels");
    let (_, _, px) = pixels(&c.s.guest).expect("shown");
    assert_eq!(px.iter().filter(|&&p| p == SHADOW).count(), 0, "no shadow on screen");
    let (_, shown) = content(&c);
    assert!(shown.iter().all(|&p| p == BLUE), "every pixel of the window shown");
}

#[test]
fn the_pointer_is_told_where_it_is_on_the_surface_from_the_geometrys_corner() {
    let mut c = viewported(G);
    let ((x0, y0), _) = content(&c);
    // 10 and 15 shown pixels in, 8 and 12 of the surface's at 1.25.
    input::motion(x0 as i32 + 10, y0 as i32 + 15);
    let evs = c.events();
    let on: Vec<(i32, i32)> = to(&evs, POINTER)
        .iter()
        .filter_map(|e| match e.opcode {
            POINTER_ENTER => Some((e.word(2) as i32, e.word(3) as i32)),
            POINTER_MOTION => Some((e.word(1) as i32, e.word(2) as i32)),
            _ => None,
        })
        .collect();
    assert!(!on.is_empty(), "the pointer entered the surface");
    let fixed = |v: i32| v * 256;
    assert!(on.iter().all(|&p| p == (fixed(G.0 + 8), fixed(G.1 + 12))), "{on:?}");
}

#[test]
fn a_change_said_in_buffer_pixels_lands_where_it_is_in_the_window() {
    let mut c = viewported(G);
    let mark = ((BOX.0 + 10, BOX.1 + 30, 20, 8), GREEN);
    attach(&mut c, 1, &picture(BLUE, Some(mark)));
    let b = mark.0;
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let (_, shown) = content(&c);
    for y in 0..BOX.3 {
        for x in 0..BOX.2 {
            let want = if (10..30).contains(&x) && (30..38).contains(&y) { GREEN } else { BLUE };
            assert_eq!(shown[(y * BOX.2 + x) as usize], want, "pixel {x},{y} of the window");
        }
    }
}

#[test]
fn a_geometry_of_the_whole_surface_or_none_of_it_shows_the_buffer_whole() {
    for g in [(0, 0, DEST.0, DEST.1), (-5, -5, DEST.0 + 10, DEST.1 + 10), (DEST.0, 0, 10, 10)] {
        let c = viewported(g);
        let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
        assert_eq!((w, h), (BW as u32, BH as u32), "{g:?}: the destination at 1.25");
        assert_eq!(c.s.guest.scene.win.origin, (0, 0), "{g:?}");
    }
}

const CONFIGURE: u16 = 0;
const SET_MAXIMIZED: u16 = 9;
const UNSET_MAXIMIZED: u16 = 10;

fn told(evs: &[Ev]) -> Vec<(u32, u32)> {
    let configures = to(evs, TOPLEVEL).into_iter().filter(|e| e.opcode == CONFIGURE);
    configures.map(|e| (e.word(0), e.word(1))).collect()
}

/// The last configure in `evs` acked and drawn as Qt draws it: a window of
/// `size` at the geometry's corner, its surface that and the shadow, the
/// viewport to the surface, the buffer at 1.25.
fn draw(c: &mut Client, evs: &[Ev], n: u32, (w, h): (i32, i32)) {
    let serial = to(evs, XDG_SURFACE).last().expect("a configure").word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    let surface = (w + DEST.0 - G.2, h + DEST.1 - G.3);
    c.req(XDG_SURFACE, 3, &[A::I(G.0), A::I(G.1), A::I(w), A::I(h)]);
    c.req(VIEWPORT, 2, &[A::I(surface.0), A::I(surface.1)]);
    let (bw, bh) = (surface.0 * 5 / 4, surface.1 * 5 / 4);
    c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (bw, bh), BLUE);
    c.req(SURFACE, 1, &[A::U(101 + 2 * n), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

#[test]
fn maximized_and_restored_it_is_told_its_windows_size_not_its_shadows() {
    let mut c = viewported(G);
    c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
    let evs = c.events();
    let &(mw, mh) = told(&evs).last().expect("told maximized");
    draw(&mut c, &evs, 1, (mw as i32, mh as i32));
    c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
    let evs = c.events();
    assert_eq!(told(&evs).last(), Some(&(G.2 as u32, G.3 as u32)), "the window's size it had");
    draw(&mut c, &evs, 2, (G.2, G.3));
    let (w, h, _) = c.s.guest.scene.win.shape.expect("shaped");
    assert_eq!((w, h), (BOX.2 as u32, BOX.3 as u32), "back at its own size, not grown");
}

#[test]
fn a_size_whose_shown_pixels_round_up_is_restored_to_itself_not_a_pixel_less() {
    // 802 at 1.25 is 1002.5 shown pixels, drawn as 1003: the window is
    // still 802 wide, and a restore tells it so.
    let mut c = viewported(G);
    let cycle = |c: &mut Client, n: u32, size: (i32, i32)| {
        c.req(TOPLEVEL, SET_MAXIMIZED, &[]);
        let evs = c.events();
        let &(mw, mh) = told(&evs).last().expect("told maximized");
        draw(c, &evs, n, (mw as i32, mh as i32));
        c.req(TOPLEVEL, UNSET_MAXIMIZED, &[]);
        let evs = c.events();
        draw(c, &evs, n + 1, size);
        told(&evs).last().copied()
    };
    cycle(&mut c, 3, (802, 602));
    assert_eq!(cycle(&mut c, 5, (802, 602)), Some((802, 602)), "the size it was drawn at");
    assert_eq!(cycle(&mut c, 7, (802, 602)), Some((802, 602)), "and again");
}
