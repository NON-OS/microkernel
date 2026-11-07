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

//! A commit whose client said two boxes or more changed far apart: each
//! is read out of the guest alone and the compositor told them apart
//! (OP_PRESENT_LAYER_PARTS, OP_LAYER_CONTENT_PARTS), so neither the rows
//! between them are read nor composed again. Whatever boxes are said, the
//! window shows the picture the client drew and no stale pixel between
//! them, framed or full screen, copied or pinned.

use nonos_app_skeleton::desk::{self, Call};
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::host::dump::pixels;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::guest_reads;

const W: i32 = 320;
const H: i32 = 200;
const BLUE: u32 = 0xFF20_40A0;
/// What a buffer holds outside the boxes it said changed: an older frame.
const STALE: u32 = 0xFFFF_00FF;

type Box4 = (i32, i32, i32, i32);

/// A toplevel showing `W` by `H` of blue, full screen when `full`.
fn window(full: bool) -> Client {
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("apart")]);
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
    commit(&mut c, 99, &vec![BLUE; (W * H) as usize], &[(0, 0, W, H)]);
    c
}

/// Buffer `n` holding `px`, attached with each of `boxes` said damaged in
/// its own request, and committed.
fn commit(c: &mut Client, n: u32, px: &[u32], boxes: &[Box4]) {
    let (pool, buffer) = (100 + 2 * n, 101 + 2 * n);
    let bytes: Vec<u8> = px.iter().flat_map(|p| p.to_le_bytes()).collect();
    // SAFETY: a new memfd, written and sized, handed to the session with
    // the request.
    let fd = unsafe {
        let fd = libc::memfd_create(c"apart".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, bytes.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, bytes.as_ptr().cast(), bytes.len()), bytes.len() as isize);
        fd
    };
    c.req_fds(SHM, 0, &[A::U(pool), A::I(bytes.len() as i32)], &[fd]);
    c.req(pool, 0, &[A::U(buffer), A::I(0), A::I(W), A::I(H), A::I(W * 4), A::U(0)]);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    for b in boxes {
        c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    }
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

fn inside((x, y, w, h): Box4, i: usize) -> bool {
    let (px, py) = ((i % W as usize) as i32, (i / W as usize) as i32);
    px >= x && py >= y && px < x + w && py < y + h
}

/// What the desktop was told of the last commit: the box presented, and
/// the boxes told apart inside it, if any.
fn told() -> Option<((u32, u32, u32, u32), Vec<(u32, u32, u32, u32)>)> {
    let calls = desk::with(|d| d.calls.clone());
    let at = calls.iter().rposition(|c| matches!(c, Call::Present(..) | Call::Content(..)))?;
    let shown = match &calls[at] {
        Call::Present(_, _, b) => *b,
        Call::Content(_, _, _, b, _) => *b,
        _ => return None,
    };
    let parts = match calls.get(at + 1) {
        Some(Call::Parts(p)) => p.clone(),
        _ => Vec::new(),
    };
    Some((shown, parts))
}

/// The box round all of `boxes`.
fn bound(boxes: &[Box4]) -> Box4 {
    let x0 = boxes.iter().map(|b| b.0).min().unwrap_or(0);
    let y0 = boxes.iter().map(|b| b.1).min().unwrap_or(0);
    let x1 = boxes.iter().map(|b| b.0 + b.2).max().unwrap_or(0);
    let y1 = boxes.iter().map(|b| b.1 + b.3).max().unwrap_or(0);
    (x0, y0, x1 - x0, y1 - y0)
}

/// Each of `frames` committed in turn over buffers holding stale pixels
/// outside the box round the boxes said, and the picture shown inside it
/// where no box was said, as a client drawing by its buffer's age holds:
/// the window shows the picture drawn, every time. With `between`, stale
/// pixels everywhere no box was said, which only boxes kept apart and
/// never joined leave unread. The bytes read out of the guest for each.
fn drawn(full: bool, frames: &[Vec<Box4>]) -> (Client, Vec<u64>) {
    drawn_as(full, frames, false)
}

fn drawn_as(full: bool, frames: &[Vec<Box4>], between: bool) -> (Client, Vec<u64>) {
    let mut c = window(full);
    let origin = content_at(&c);
    let mut model = vec![BLUE; (W * H) as usize];
    let mut reads = Vec::new();
    for (n, boxes) in frames.iter().enumerate() {
        let argb = 0xFF00_0000 | (0x0003_0507 * (n as u32 + 1));
        let said = |i: usize| boxes.iter().any(|&b| inside(b, i));
        let round = bound(boxes);
        let held = |i: usize| match (said(i), inside(round, i) && !between) {
            (true, _) => argb,
            (false, true) => model[i],
            (false, false) => STALE,
        };
        let px: Vec<u32> = (0..model.len()).map(held).collect();
        model.iter_mut().enumerate().filter(|(i, _)| said(*i)).for_each(|(_, p)| *p = argb);
        guest_reads::take();
        desk::with(|d| d.calls.clear());
        commit(&mut c, n as u32, &px, boxes);
        reads.push(guest_reads::take().bytes);
        let shown = content(&c, origin);
        let stale = shown.iter().filter(|&&p| p == STALE).count();
        assert_eq!(stale, 0, "frame {n} {boxes:?}: {stale} stale pixels shown (full {full})");
        let wrong = shown.iter().zip(&model).filter(|(a, b)| a != b).count();
        assert_eq!(wrong, 0, "frame {n} {boxes:?}: {wrong} pixels not the picture (full {full})");
    }
    (c, reads)
}

const ROWS: u64 = (W as u64) * 4;

/// A selection near the top and the last row, as a terminal says them.
const TWO: [Box4; 2] = [(0, 12, W, 16), (0, 184, W, 16)];

#[test]
fn two_rows_far_apart_are_read_and_told_apart_framed_or_full_screen() {
    for full in [false, true] {
        let (_, reads) = drawn(full, &[TWO.to_vec()]);
        assert_eq!(reads, [32 * ROWS], "the two boxes' rows alone (full {full})");
        let (shown, parts) = told().expect("presented");
        assert_eq!(shown.3, 188, "the box round both: rows 12 to 200 (full {full})");
        assert_eq!(parts.len(), 2, "told apart: {parts:?} (full {full})");
        let heights: Vec<u32> = parts.iter().map(|p| p.3).collect();
        assert_eq!(heights, [16, 16], "{parts:?}");
        assert!(parts.iter().all(|p| p.2 == W as u32), "whole rows: {parts:?}");
        for p in &parts {
            let x1 = shown.0 + shown.2;
            let y1 = shown.1 + shown.3;
            assert!(p.0 >= shown.0 && p.1 >= shown.1 && p.0 + p.2 <= x1 && p.1 + p.3 <= y1);
        }
    }
}

#[test]
fn the_rows_between_two_boxes_apart_are_never_read_or_shown() {
    for full in [false, true] {
        let frames = vec![TWO.to_vec(), vec![(0, 40, W, 8), (0, 150, W, 8)], TWO.to_vec()];
        let (_, reads) = drawn_as(full, &frames, true);
        assert_eq!(reads, [32 * ROWS, 16 * ROWS, 32 * ROWS], "full {full}");
    }
}

#[test]
fn rows_said_one_by_one_are_one_box_and_told_as_one() {
    let rows: Vec<Box4> = (0..8).map(|i| (0, 40 + i, W, 1)).collect();
    let (_, reads) = drawn(false, &[rows]);
    assert_eq!(reads, [8 * ROWS]);
    let (shown, parts) = told().expect("presented");
    assert_eq!(shown.3, 8);
    assert!(parts.is_empty(), "one box is the plain present: {parts:?}");
}

#[test]
fn boxes_that_overlap_or_sit_close_are_one() {
    let near = vec![(10, 10, 40, 20), (30, 20, 40, 20), (52, 10, 10, 10)];
    let (_, reads) = drawn(false, &[near]);
    let (_, parts) = told().expect("presented");
    assert!(parts.is_empty(), "one box: {parts:?}");
    assert!(reads[0] <= 30 * 60 * 4, "{reads:?}");
}

#[test]
fn five_boxes_apart_are_told_as_four_at_most_and_every_one_is_drawn() {
    // Inside the buffer's edge: a box round them all that is the whole
    // buffer is taken as all of it, as a client naming everything means.
    let five = vec![
        (1, 1, 20, 10),
        (299, 1, 20, 10),
        (150, 95, 20, 10),
        (1, 189, 20, 10),
        (299, 189, 20, 10),
    ];
    let (_, reads) = drawn(false, &[five]);
    let (_, parts) = told().expect("presented");
    assert!((2..=4).contains(&parts.len()), "{parts:?}");
    assert!(reads[0] < (W * H * 4) as u64 / 2, "the boxes, not the window: {reads:?}");
}

/// A small generator, so a failing run names its seed.
struct Rng(u64);

impl Rng {
    fn next(&mut self, n: u32) -> i32 {
        self.0 =
            self.0.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1_442_695_040_888_963_407);
        ((self.0 >> 33) % u64::from(n)) as i32
    }
}

#[test]
fn any_boxes_over_both_buffers_show_the_picture_drawn_and_no_stale_pixel() {
    for (seed, full) in [(1u64, false), (2, true), (3, false), (4, true)] {
        let mut r = Rng(seed);
        let frames: Vec<Vec<Box4>> = (0..24)
            .map(|_| {
                (0..1 + r.next(6))
                    .map(|_| {
                        let (x, y) = (r.next(W as u32 - 1), r.next(H as u32 - 1));
                        (x, y, 1 + r.next((W - x) as u32), 1 + r.next((H - y) as u32))
                    })
                    .collect()
            })
            .collect();
        let _ = drawn(full, &frames);
    }
}

fn pins(allow: bool) {
    nonos_libc::world::with(|w| w.allow_pins = allow);
}

/// A framed toplevel shown whole from buffer 101, with 101 and 103 painted.
fn pinned_window() -> Client {
    set_clock(1_000);
    desk::with(|d| (d.width, d.height) = (W as u32 + 256, H as u32 + 256));
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("apart")]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (W, H), BLUE + n);
    }
    for _ in 0..2 {
        attach(&mut c, 101, &[(0, 0, W, H)]);
    }
    c
}

fn attach(c: &mut Client, buffer: u32, boxes: &[Box4]) {
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    for b in boxes {
        c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    }
    c.req(SURFACE, 6, &[]);
    c.events();
}

#[test]
fn a_pinned_swapchain_tells_the_compositor_its_boxes_apart_and_reads_nothing() {
    pins(true);
    let mut c = pinned_window();
    attach(&mut c, 103, &[(0, 0, W, H)]);
    for i in 0..6u32 {
        desk::with(|d| d.calls.clear());
        guest_reads::take();
        attach(&mut c, if i % 2 == 0 { 101 } else { 103 }, &TWO);
        assert_eq!(guest_reads::take().bytes, 0, "a pin is read in place");
        let calls = desk::with(|d| d.calls.clone());
        assert!(calls.iter().any(|c| matches!(c, Call::Content(..))), "pinned: {calls:?}");
        let (shown, parts) = told().expect("told");
        assert_eq!(shown, (0, 12, W as u32, 188), "the box round both");
        assert_eq!(parts, [(0, 12, W as u32, 16), (0, 184, W as u32, 16)], "told apart");
    }
    pins(false);
}
