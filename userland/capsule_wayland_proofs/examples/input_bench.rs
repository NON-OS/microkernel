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

//! What input on a Linux window costs the Wayland bridge, on the host: the
//! shipped server (`crate::linux::wayland`) serving a framed `W` by `H`
//! window on a 1920x1080 display, each frame timed from the router's event
//! to the bridge's last answer, the client's redraw included where the
//! client would draw one.
//!
//! - drag: the title bar held and moved 8 pixels a frame.
//! - resize: the corner pressed, moved 8 pixels and let go, the configure
//!   acked and a buffer of the new size committed whole.
//! - scroll: a wheel step on the content, the pane under a toolbar redrawn.
//! - hover: the pointer moved over the content, a 120x40 button lit.
//! - click: a press and a release on the content, the button redrawn.
//!
//! Each line gives the median and the 99th of the frames, and the calls
//! the bridge made on the desktop (compositor and window manager) a frame.
//!
//!     cargo +nightly-2026-01-16 run --release --example input_bench [W H]

use std::time::Instant;

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input::{motion, post, wheel, Kind};

const FRAMES: usize = 256;
const RUNS: usize = 5;
const POINTER: u32 = 14;
const STEP: i32 = 8;
/// The pool and buffer ids a resize draws into, one a frame in turn.
const RESIZED: u32 = 200;

/// A shown, framed `w` by `h` window with a pointer, drawn into two buffers.
fn window(w: i32, h: i32) -> Client {
    let mut c = toplevel(6);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(TOPLEVEL, 2, &[A::S("bench")]);
    c.req(SURFACE, 6, &[]);
    let configure = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(configure)]);
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, (w, h), 0xFF20_40A0 + n);
    }
    c.req(SURFACE, 1, &[A::U(101), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

/// The window's size on screen, frame and all.
fn framed() -> (i32, i32) {
    desk::with(|d| d.layers.last().map_or((0, 0), |l| (l.width as i32, l.height as i32)))
}

fn calls() -> usize {
    desk::with(|d| d.calls.len())
}

/// The other buffer of the two, damaged at `b` and committed.
fn commit(c: &mut Client, i: usize, b: (i32, i32, i32, i32)) {
    let buffer = 101 + 2 * (i as u32 % 2);
    c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(b.0), A::I(b.1), A::I(b.2), A::I(b.3)]);
    c.req(SURFACE, 6, &[]);
}

#[derive(Clone, Copy)]
enum Scene {
    Drag,
    Resize,
    Scroll,
    Hover,
    Click,
}

const SCENES: [(&str, Scene); 5] = [
    ("drag", Scene::Drag),
    ("resize", Scene::Resize),
    ("scroll", Scene::Scroll),
    ("hover", Scene::Hover),
    ("click", Scene::Click),
];

/// What frame `i` of `scene` asks, timed; the untimed work a client does
/// between (drawing a new buffer) is left out.
fn frame(c: &mut Client, scene: Scene, i: usize, (w, h): (i32, i32)) -> f64 {
    let (fw, fh) = framed();
    let on = (fw / 2, fh / 2);
    let button = (w / 2, h / 2, 120, 40);
    match scene {
        Scene::Drag => {
            // Laid from where the press was, so the window goes out and back.
            let k = i as i32 % 128;
            let x = fw / 2 + STEP * k.min(128 - k);
            let at = Instant::now();
            motion(x, 20 + STEP * (i as i32 % 2));
            c.events();
            at.elapsed().as_secs_f64() * 1e6
        }
        Scene::Resize => {
            let corner = (fw - 2, fh - 2);
            // Out for 32 frames and back for 32, so it stays on the screen.
            let d = if i % 64 < 32 { STEP } else { -STEP };
            let to_ = (corner.0 + d, corner.1 + d);
            let at = Instant::now();
            motion(corner.0, corner.1);
            post(Kind::ButtonDown, 0, 1, corner, (0, 0));
            motion(to_.0, to_.1);
            post(Kind::ButtonUp, 0, 1, to_, (0, 0));
            let evs = c.events();
            let took = at.elapsed().as_secs_f64() * 1e6;
            let told = to(&evs, TOPLEVEL).into_iter().rfind(|e| e.opcode == 0);
            let size = told.map_or((w, h), |e| (e.word(0) as i32, e.word(1) as i32));
            let Some(serial) = to(&evs, XDG_SURFACE).last().map(|e| e.word(0)) else {
                return took;
            };
            let id = RESIZED + 2 * (i as u32 % 2);
            if i >= 2 {
                // The buffer and pool of two frames ago, shown no longer.
                c.req(id + 1, 0, &[]);
                c.req(id, 1, &[]);
            }
            c.painted(SHM, id, id + 1, size, 0xFF20_40A0);
            let at = Instant::now();
            c.req(XDG_SURFACE, 4, &[A::U(serial)]);
            c.req(SURFACE, 1, &[A::U(id + 1), A::I(0), A::I(0)]);
            c.req(SURFACE, 6, &[]);
            c.events();
            took + at.elapsed().as_secs_f64() * 1e6
        }
        Scene::Scroll => {
            let at = Instant::now();
            wheel(0, 1, on);
            c.events();
            commit(c, i, (0, 48, w, h - 48));
            c.events();
            at.elapsed().as_secs_f64() * 1e6
        }
        Scene::Hover => {
            let at = Instant::now();
            motion(on.0 + (i as i32 % 2), on.1);
            c.events();
            commit(c, i, button);
            c.events();
            at.elapsed().as_secs_f64() * 1e6
        }
        Scene::Click => {
            let at = Instant::now();
            post(Kind::ButtonDown, 0, 1, on, (0, 0));
            post(Kind::ButtonUp, 0, 1, on, (0, 0));
            c.events();
            commit(c, i, button);
            c.events();
            at.elapsed().as_secs_f64() * 1e6
        }
    }
}

/// The median and the 99th of `FRAMES` frames, in microseconds, and the
/// desktop calls a frame.
fn run(scene: Scene, (w, h): (i32, i32)) -> (f64, f64, f64) {
    desk::with(|d| *d = desk::Desk { width: 1920, height: 1080, ..desk::Desk::default() });
    let mut c = window(w, h);
    let (fw, fh) = framed();
    match scene {
        Scene::Drag => {
            motion(fw / 2, 20);
            post(Kind::ButtonDown, 0, 1, (fw / 2, 20), (0, 0));
        }
        Scene::Hover | Scene::Click | Scene::Scroll => motion(fw / 2, fh / 2),
        Scene::Resize => {}
    }
    c.events();
    let before = calls();
    let mut t: Vec<f64> = (0..FRAMES).map(|i| frame(&mut c, scene, i, (w, h))).collect();
    let made = calls().saturating_sub(before) as f64 / FRAMES as f64;
    t.sort_by(f64::total_cmp);
    (t[FRAMES / 2], t[FRAMES * 99 / 100], made)
}

fn main() {
    let args: Vec<i32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let size = match args[..] {
        [w, h] => (w, h),
        _ => (1280, 720),
    };
    for (name, scene) in SCENES {
        let mut best = (f64::MAX, f64::MAX, 0.0);
        for _ in 0..RUNS {
            let (p50, p99, made) = run(scene, size);
            best = (best.0.min(p50), best.1.min(p99), made);
        }
        let (w, h) = size;
        println!(
            "{w}x{h} Linux window, {name}: p50 {:.1} us, p99 {:.1} us a frame, {:.1} desktop calls",
            best.0, best.1, best.2
        );
    }
}
