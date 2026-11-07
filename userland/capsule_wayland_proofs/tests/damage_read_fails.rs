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

//! A damaged box whose read out of the guest fails (damage_place.rs): the
//! guest's pages went from under the server, as when it unmaps its pool or
//! exits between the commit and the read, all of them or those past the
//! box's middle row, so some rows land in the picture before the read fails.
//! However the box is read, a row at a time into the picture, in one read or
//! as a span, nothing of it is presented, and the next commit shows the client's picture with no pixel
//! of the buffer that failed, framed or full screen.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::linux::guest::note_cut;

const W: i32 = 320;
const H: i32 = 200;
const BLUE: u32 = 0xFF20_40A0;
/// The failed buffer's box: never to be shown.
const LOST: u32 = 0xFFFF_0000;
const NEXT: u32 = 0xFF00_7700;

type Box4 = (i32, i32, i32, i32);

/// A box read a row at a time, one read whole across, and one as a span.
const BOXES: [Box4; 3] = [(150, 90, 40, 12), (0, 30, W, 120), (80, 20, W - 80, H - 20)];

/// A toplevel showing `W` by `H` of blue, full screen when `full`.
fn window(full: bool) -> Client {
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("lost")]);
    if full {
        c.req(TOPLEVEL, 11, &[A::U(0)]);
    }
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.painted(SHM, POOL, BUFFER, (W, H), BLUE);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    // Shown once more, whole: the first frame is painted again as the
    // window takes focus, so the commits after are drawn as their box.
    buffer(&mut c, 99, &painted(None, 0));
    commit(&mut c, 99, (0, 0, W, H));
    c
}

/// Blue with box `b` of `argb`.
fn painted(b: Option<Box4>, argb: u32) -> Vec<u32> {
    let inside = |i: usize| {
        let (x, y) = ((i % W as usize) as i32, (i / W as usize) as i32);
        b.is_some_and(|(bx, by, bw, bh)| x >= bx && y >= by && x < bx + bw && y < by + bh)
    };
    (0..(W * H) as usize).map(|i| if inside(i) { argb } else { BLUE }).collect()
}

/// Buffer `n` holding `px` in a pool of its own: where the server reads it.
fn buffer(c: &mut Client, n: u32, px: &[u32]) -> u64 {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    let bytes: Vec<u8> = px.iter().flat_map(|p| p.to_le_bytes()).collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"lost".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(W), A::I(H), A::I(W * 4), A::U(0)]);
    c.s.guest.memfd_spans.last().expect("the pool is mapped").at
}

/// Buffer `n` attached with box `b` said damaged, and committed.
fn commit(c: &mut Client, n: u32, b: Box4) {
    c.req(SURFACE, 1, &[A::U(101 + 2 * n), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
    c.events();
}

/// The guest's `W` by `H` as the window shows them, from where blue was.
fn content(c: &Client, (x0, y0): (usize, usize)) -> Vec<u32> {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let w = w as usize;
    (0..H as usize)
        .flat_map(|y| (0..W as usize).map(move |x| (y, x)))
        .map(|(y, x)| px[(y0 + y) * w + x0 + x])
        .collect()
}

fn content_at(c: &Client) -> (usize, usize) {
    let (w, _, px) = pixels(&c.s.guest).expect("shown");
    let at = px.iter().position(|&p| p == BLUE).expect("the guest's pixels are shown");
    (at % w as usize, at / w as usize)
}

fn presented() -> usize {
    desk::with(|d| d.calls.iter().filter(|c| matches!(c, Call::Present(..))).count())
}

fn a_failed_read_is_never_shown(full: bool) {
    let cuts = BOXES.into_iter().flat_map(|b| [(b, 0), (b, ((b.1 + b.3 / 2) * W * 4) as u64)]);
    for (b, keep) in cuts {
        let mut c = window(full);
        let origin = content_at(&c);
        let at = buffer(&mut c, 0, &painted(Some(b), LOST));
        note_cut(at, keep);
        desk::with(|d| d.calls.clear());
        commit(&mut c, 0, b);
        assert_eq!(presented(), 0, "box {b:?} cut at {keep} (full {full}): not presented");
        let lost = content(&c, origin).iter().filter(|&&p| p == LOST).count();
        assert_eq!(lost, 0, "box {b:?} (full {full}): {lost} pixels of it shown");
        // The client's next picture, the box it failed in left blue.
        let next = (b.0, b.1, b.2.min(8), b.3.min(8));
        let picture = painted(Some(next), NEXT);
        buffer(&mut c, 1, &picture);
        commit(&mut c, 1, next);
        let shown = content(&c, origin);
        let wrong = shown.iter().zip(&picture).filter(|(a, b)| a != b).count();
        assert_eq!(wrong, 0, "box {b:?} (full {full}): {wrong} pixels after not the picture");
        if !full {
            let client = &c.s.guest.scene.win.frame.client;
            let kept: Vec<u32> = client
                .chunks_exact(4)
                .take((W * H) as usize)
                .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]]))
                .collect();
            assert_eq!(kept, picture, "box {b:?}: the frame paints again from the picture");
        }
    }
}

#[test]
fn a_framed_window_never_shows_a_box_whose_read_failed() {
    a_failed_read_is_never_shown(false);
}

#[test]
fn a_full_screen_window_never_shows_a_box_whose_read_failed() {
    a_failed_read_is_never_shown(true);
}
