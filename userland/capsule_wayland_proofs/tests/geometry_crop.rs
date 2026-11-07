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

//! A client that draws its own shadow round its window and says where the
//! window is (xdg_surface.set_window_geometry), as Firefox does: an 852x652
//! buffer for an 800x600 window at 26,23. The window shows that box alone
//! (present/crop.rs); the pointer, and what the client says changed, are
//! reckoned from its corner.

use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::input;

const BW: i32 = 852;
const BH: i32 = 652;
const G: (i32, i32, i32, i32) = (26, 23, 800, 600);
const SHADOW: u32 = 0xFF10_1010;
const BLUE: u32 = 0xFF20_40A0;
const GREEN: u32 = 0xFF20_A040;
const POINTER: u32 = 40;
const POINTER_ENTER: u16 = 0;
const POINTER_MOTION: u16 = 2;

/// The buffer: the shadow outside the geometry, `inside` within, and the
/// box `b` (buffer pixels) of `mark`.
fn picture(inside: u32, b: Option<((i32, i32, i32, i32), u32)>) -> Vec<u32> {
    let within = |x: i32, y: i32, r: (i32, i32, i32, i32)| {
        x >= r.0 && y >= r.1 && x < r.0 + r.2 && y < r.1 + r.3
    };
    (0..BH)
        .flat_map(|y| (0..BW).map(move |x| (x, y)))
        .map(|(x, y)| match b {
            Some((r, mark)) if within(x, y, r) => mark,
            _ if within(x, y, G) => inside,
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
        let fd = libc::memfd_create(c"shadowed".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(BW), A::I(BH), A::I(BW * 4), A::U(0)]);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
}

fn shadowed() -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(XDG_SURFACE, 3, &[A::I(G.0), A::I(G.1), A::I(G.2), A::I(G.3)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    attach(&mut c, 0, &picture(BLUE, None));
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The window's corner of content, and the content as shown.
fn content(c: &Client) -> ((usize, usize), Vec<u32>) {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let w = w as usize;
    let at = px.iter().position(|&p| p == BLUE || p == GREEN).expect("content shown");
    let (x0, y0) = (at % w, at / w);
    let rows = (0..G.3 as usize).flat_map(|y| (0..G.2 as usize).map(move |x| (y, x)));
    ((x0, y0), rows.map(|(y, x)| px[(y0 + y) * w + x0 + x]).collect())
}

#[test]
fn the_window_is_its_geometry_and_the_shadow_is_not_shown() {
    let c = shadowed();
    let (w, h, stride) = c.s.guest.scene.win.shape.expect("shaped");
    assert_eq!((w, h, stride), (800, 600, BW as u32 * 4), "the geometry's size, the buffer's rows");
    assert_eq!(c.s.guest.scene.win.origin, (26, 23));
    let (_, _, px) = pixels(&c.s.guest).expect("shown");
    assert_eq!(px.iter().filter(|&&p| p == SHADOW).count(), 0, "no shadow on screen");
    let (_, shown) = content(&c);
    assert!(shown.iter().all(|&p| p == BLUE), "every pixel of the geometry shown");
}

#[test]
fn the_pointer_is_told_where_it_is_on_the_surface_from_the_geometrys_corner() {
    let mut c = shadowed();
    let ((x0, y0), _) = content(&c);
    input::motion(x0 as i32 + 5, y0 as i32 + 7);
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
    assert!(on.iter().all(|&p| p == (fixed(26 + 5), fixed(23 + 7))), "{on:?}");
}

#[test]
fn a_change_said_in_buffer_pixels_lands_where_it_is_in_the_window() {
    let mut c = shadowed();
    let mark = ((G.0 + 10, G.1 + 30, 20, 8), GREEN);
    attach(&mut c, 1, &picture(BLUE, Some(mark)));
    let b = mark.0;
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let (_, shown) = content(&c);
    for y in 0..G.3 {
        for x in 0..G.2 {
            let want = if (10..30).contains(&x) && (30..38).contains(&y) { GREEN } else { BLUE };
            assert_eq!(shown[(y * G.2 + x) as usize], want, "pixel {x},{y} of the window");
        }
    }
    // A change in the shadow alone shows nothing.
    attach(&mut c, 2, &picture(BLUE, Some(((0, 0, 8, 8), GREEN))));
    c.req(SURFACE, 9, &[A::I(0), A::I(0), A::I(8), A::I(8)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let (_, again) = content(&c);
    assert_eq!(again, shown);
}
