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

//! What a frame costs the driver at 1920 x 1080, over its own code and the
//! host model: the bytes TRANSFER_TO_HOST_2D copies, the commands on each
//! queue, the passes the host made over the control ring (each a VM exit
//! on a real host) and the time from the request to its reply, which
//! includes the model's copy of the pixels as a host makes it. Pointer
//! motion is timed both ways: the composited pointer's two boxes flushed,
//! and the cursor plane's MOVE_CURSOR. Run with
//! `cargo test --release present_bench -- --ignored --nocapture`.

use std::time::Instant;

use nonos_gfx_wire::body::cursor::{NEW_SHAPE, VISIBLE};
use nonos_gfx_wire::{ops, Surface};

use super::ask::{send, words};
use super::cursor_kit::{at, cursor_rig_on, point, shaped, CursorRig};
use super::kit::draw;
use super::rig::rig_with;

const FRAMES: usize = 600;
const SCREEN: (u32, u32) = (1920, 1080);

/// The rectangles a scene flushes on frame `i`.
type Scene = fn(u32) -> Vec<[u32; 4]>;

fn pointer(i: u32) -> Vec<[u32; 4]> {
    let (x, y) = (100 + i * 2 % 1600, 100 + i % 800);
    vec![[x - 2, y - 1, 64, 64], [x, y, 64, 64]]
}

fn drag(i: u32) -> Vec<[u32; 4]> {
    vec![[i * 8 % 1000, 200, 808, 600]]
}

fn resize(i: u32) -> Vec<[u32; 4]> {
    let grow = i * 8 % 400;
    vec![[100, 100, 1288 + grow.min(400), 728]]
}

fn scroll(_: u32) -> Vec<[u32; 4]> {
    vec![[320, 180, 1280, 720]]
}

fn full(_: u32) -> Vec<[u32; 4]> {
    vec![[0, 0, SCREEN.0, SCREEN.1]]
}

struct Tally {
    bytes: u64,
    control: usize,
    rounds: u32,
    cursor: usize,
    us: Vec<f64>,
}

fn counts(c: &CursorRig) -> (u64, usize, u32, usize) {
    let h = c.r.host.borrow();
    (h.moved, h.log.len(), h.rounds, c.cursor.borrow().said.len())
}

fn timed(c: &mut CursorRig, mut frame: impl FnMut(&mut CursorRig, u32)) -> Tally {
    let before = counts(c);
    let mut us = Vec::with_capacity(FRAMES);
    for i in 0..FRAMES as u32 {
        let t = Instant::now();
        frame(c, i);
        us.push(t.elapsed().as_secs_f64() * 1e6);
    }
    let after = counts(c);
    us.sort_by(|a, b| a.total_cmp(b));
    Tally {
        bytes: after.0 - before.0,
        control: after.1 - before.1,
        rounds: after.2 - before.2,
        cursor: after.3 - before.3,
        us,
    }
}

fn flush(c: &mut CursorRig, s: &Surface, rects: &[[u32; 4]]) {
    let mut w = vec![s.serial, rects.len() as u32];
    rects.iter().for_each(|r| w.extend_from_slice(r));
    assert_eq!(send(&mut c.r, ops::FLUSH, 0, &words(&w)).0, 0, "flushed");
}

fn report(name: &str, t: &Tally) {
    let n = FRAMES as f64;
    let pick = |q: f64| t.us[((t.us.len() - 1) as f64 * q) as usize];
    println!(
        "{name:<32} {:>9.0} B/frame  ctrl {:>4.1}  ctrl exits {:>4.1}  cursor {:>3.1}  p50 {:>7.1} us  p99 {:>7.1} us",
        t.bytes as f64 / n,
        t.control as f64 / n,
        t.rounds as f64 / n,
        t.cursor as f64 / n,
        pick(0.5),
        pick(0.99),
    );
}

#[test]
#[ignore = "a benchmark, run by hand"]
fn present_bench() {
    let mut c = cursor_rig_on(rig_with(&[SCREEN]));
    let s = draw(&mut c.r, 0, 0x0020_3040);
    shaped(&mut c.r, 0xFF10_2030);
    assert_eq!(point(&mut c.r, at(VISIBLE | NEW_SHAPE, (100, 100), (0, 0))), 0);
    let scenes: [(&str, Scene); 5] = [
        ("pointer, composited boxes", pointer),
        ("drag 800x600 by 8 px", drag),
        ("resize 1280x720 by 8 px", resize),
        ("scroll 1280x720", scroll),
        ("full screen", full),
    ];
    for (name, scene) in scenes {
        let t = timed(&mut c, |c, i| flush(c, &s, &scene(i)));
        report(name, &t);
    }
    let t = timed(&mut c, |c, i| {
        let (x, y) = (100 + (i * 2 % 1600) as i32, 100 + (i % 800) as i32);
        assert_eq!(point(&mut c.r, at(VISIBLE, (x, y), (0, 0))), 0);
    });
    assert_eq!((t.bytes, t.control), (0, 0), "the plane moves with nothing on the control queue");
    report("pointer, cursor plane", &t);
}
