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

//! A commit's damaged box read out of the guest (damage_box.rs,
//! damage_place.rs): a full-width one in one read, any other as its rows at
//! the stride in one strided call (guest/mem_rows.rs), each straight into
//! the picture. However it is read, only the box reaches the
//! window: a client's buffer that holds an older frame outside the box it
//! said changed never shows it, framed or full screen, ARGB or XRGB.

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
const BLUE: u32 = 0xFF20_40A0;
/// What a buffer holds outside the box it said changed: an older frame.
const STALE: u32 = 0xFFFF_00FF;
const XRGB: u32 = 1;

/// A toplevel showing `W` by `H` of blue, full screen when `full`.
fn window(full: bool) -> Client {
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("boxes")]);
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
    // Shown once more: the window's first frame is painted again as it
    // takes focus, and a commit then is shown whole.
    commit(&mut c, 99, 0, &vec![BLUE; (W * H) as usize], (0, 0, W, H));
    c
}

/// Buffer `n`, of `format`, holding `px`, attached with the box `b` said
/// damaged and committed.
fn commit(c: &mut Client, n: u32, format: u32, px: &[u32], b: (i32, i32, i32, i32)) {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    let bytes: Vec<u8> = px.iter().flat_map(|p| p.to_le_bytes()).collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"boxes".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(W), A::I(H), A::I(W * 4), A::U(format)]);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
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

fn inside((x, y, w, h): (i32, i32, i32, i32), i: usize) -> bool {
    let (px, py) = ((i % W as usize) as i32, (i / W as usize) as i32);
    px >= x && py >= y && px < x + w && py < y + h
}

/// Boxes of each way of reading: a button, a caret, a chat pane and a
/// near-full box as rows at the stride, a full-width scroll in one read.
const BOXES: [(i32, i32, i32, i32); 5] = [
    (150, 90, 40, 12),
    (7, 33, 2, 18),
    (0, 30, W, 120),
    (80, 20, W - 80, H - 20),
    (1, 1, W - 2, H - 2),
];

/// Each box committed in turn over a buffer holding stale pixels outside
/// it: the window shows the picture the client drew, and no stale pixel.
fn only_the_box_is_shown(full: bool, format: u32) {
    let mut c = window(full);
    let origin = content_at(&c);
    let mut model = vec![BLUE; (W * H) as usize];
    for (n, b) in BOXES.into_iter().enumerate() {
        let argb = 0x0010_2030 * (n as u32 + 1);
        let alpha = if format == XRGB { 0 } else { 0xFF00_0000 };
        let px: Vec<u32> =
            (0..model.len()).map(|i| if inside(b, i) { alpha | argb } else { STALE }).collect();
        model.iter_mut().enumerate().filter(|(i, _)| inside(b, *i)).for_each(|(_, p)| {
            *p = 0xFF00_0000 | argb;
        });
        guest_reads::take();
        commit(&mut c, n as u32, format, &px, b);
        let read = guest_reads::take().bytes;
        assert!(read < (W * H * 4) as u64, "box {b:?} read as a box, not whole: {read}");
        let shown = content(&c, origin);
        let stale = shown.iter().filter(|&&p| p == STALE).count();
        assert_eq!(stale, 0, "box {b:?}: {stale} stale pixels shown (full {full})");
        let wrong = shown.iter().zip(&model).filter(|(a, b)| a != b).count();
        assert_eq!(wrong, 0, "box {b:?}: {wrong} pixels not the picture (full {full})");
        if !full {
            // What the frame paints again from, as a control lit or a title
            // set does: the same picture.
            let kept = kept(&c);
            let wrong = kept.iter().zip(&model).filter(|(a, b)| a != b).count();
            assert_eq!(wrong, 0, "box {b:?}: {wrong} pixels kept not the picture");
        }
    }
}

/// The guest's pixels a framed window keeps for its frame's repaints.
fn kept(c: &Client) -> Vec<u32> {
    let client = &c.s.guest.scene.win.frame.client;
    client
        .chunks_exact(4)
        .take((W * H) as usize)
        .map(|p| u32::from_le_bytes([p[0], p[1], p[2], p[3]]))
        .collect()
}

#[test]
fn a_framed_window_shows_only_the_box_said_changed() {
    only_the_box_is_shown(false, 0);
}

#[test]
fn a_full_screen_window_shows_only_the_box_said_changed() {
    only_the_box_is_shown(true, 0);
}

#[test]
fn an_xrgb_box_is_shown_opaque_and_alone_however_it_is_read() {
    only_the_box_is_shown(false, XRGB);
    only_the_box_is_shown(true, XRGB);
}

/// Wide boxes moved and grown frame after frame over the window's two
/// buffers in turn, a narrow one between them, each over a guest buffer
/// holding stale pixels outside it: whichever buffer the window draws in
/// and whatever the one shown holds, the bytes between a box's rows are the
/// picture's, never the guest's.
#[test]
fn a_full_screen_box_keeps_the_picture_between_its_rows_over_both_buffers() {
    let mut c = window(true);
    let origin = content_at(&c);
    let mut model = vec![BLUE; (W * H) as usize];
    for n in 0..16u32 {
        let k = n as i32;
        let b = match n % 4 {
            3 => (10 + k, 5, 30, 40),
            _ => (40 + 3 * k, 8 + 2 * k, W - 60 - 3 * k, 60 + k),
        };
        let argb = 0x0001_0203 * (n + 3);
        let px: Vec<u32> = (0..model.len())
            .map(|i| if inside(b, i) { 0xFF00_0000 | argb } else { STALE })
            .collect();
        model.iter_mut().enumerate().filter(|(i, _)| inside(b, *i)).for_each(|(_, p)| {
            *p = 0xFF00_0000 | argb;
        });
        commit(&mut c, n, 0, &px, b);
        let shown = content(&c, origin);
        let wrong = shown.iter().zip(&model).filter(|(a, b)| a != b).count();
        assert_eq!(wrong, 0, "frame {n}, box {b:?}: {wrong} pixels not the picture");
    }
}

/// Each box takes one kernel call and reads only its own bytes, never the
/// bytes between its rows: a button, a chat pane, a full-width scroll.
#[test]
fn a_box_reads_its_own_bytes_in_one_call() {
    for full in [false, true] {
        let mut c = window(full);
        let px = vec![BLUE; (W * H) as usize];
        let pane = (80, 20, W - 80, H - 20);
        for (n, (b, calls, bytes)) in [
            ((150, 90, 40, 12), 1, 40 * 12 * 4),
            (pane, 1, (W - 80) * (H - 20) * 4),
            ((0, 30, W, 120), 1, W * 120 * 4),
        ]
        .into_iter()
        .enumerate()
        {
            guest_reads::take();
            commit(&mut c, n as u32, 0, &px, b);
            let r = guest_reads::take();
            assert_eq!((r.calls, r.bytes), (calls, bytes as u64), "{b:?} full {full}");
        }
    }
}
