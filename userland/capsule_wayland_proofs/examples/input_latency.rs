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

//! How long the Wayland bridge holds what a person did before it is shown,
//! on the host and on the bridge's own clock (the session's, set by hand):
//! the shipped server, a client drawing a 400 by 300 window, the desk and
//! the kernel as doubles. Three cases, each printed as one line:
//!
//! - hover: the longest the serve loop is asked to sleep inside one input
//!   pass that lights a frame control while the compositor holds the other
//!   buffer, so the input behind it waits as long.
//! - feedback: for a frame the compositor shows d ms after its present
//!   (d from 0 to 16), how long after it is shown the client is told;
//!   mean and worst.
//! - click: how far from the pointer a click lands for a client that left
//!   a mebibyte unread while the pointer moved 90 pixels.
//!
//!     cargo +nightly-2026-01-16 run --release --example input_latency

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::input;
use nonos_capsule_wayland_proofs::linux::call::set_clock;
use nonos_capsule_wayland_proofs::linux::wayland::pump;
use nonos_toolkit::decorations::{hit_test_at, DecorationHit};

const PRESENTATION: u32 = 30;
const POINTER: u32 = 14;
const FEEDBACK: u32 = 40;
const PRESENTED: u16 = 1;
const MOTION: u16 = 2;
const BUTTON: u16 = 3;

fn window(presentation: bool) -> Client {
    let mut c = Client::new();
    let mut globals = vec![
        ("wl_compositor", 6, COMPOSITOR),
        ("xdg_wm_base", 6, WM_BASE),
        ("wl_seat", 9, SEAT),
        ("wl_shm", 1, SHM),
    ];
    if presentation {
        globals.push(("wp_presentation", 1, PRESENTATION));
    }
    c.bind(REGISTRY, &globals);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.req(SURFACE, 6, &[]);
    let serial = to(&c.events(), XDG_SURFACE)[0].word(0);
    c.req(XDG_SURFACE, 4, &[A::U(serial)]);
    c.buffer(SHM, POOL, BUFFER, (400, 300));
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

fn hover() -> u64 {
    let mut c = window(false);
    desk::with(|d| d.hold = true);
    c.buffer(SHM, 102, 103, (400, 300));
    c.req(SURFACE, 1, &[A::U(103), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let (w, h) = c.s.guest.scene.win.bufs.shape().into_wh();
    let q = c.s.guest.scene.win.frame.quarters;
    let close = (0..w)
        .flat_map(|x| (0..h.min(64)).map(move |y| (x, y)))
        .find(|&(x, y)| hit_test_at(w, h, false, x, y, q) == DecorationHit::CloseButton)
        .expect("a close button");
    nonos_libc::world::with(|world| world.longest_wait_ms = 0);
    input::motion(close.0 as i32, close.1 as i32);
    pump(&mut c.s.guest);
    desk::with(|d| d.hold = false);
    nonos_libc::world::with(|world| world.longest_wait_ms)
}

/// How long after a frame shown `shown_after` ms past its present the
/// client is told, the bridge waking when its pace says.
fn feedback(shown_after: u64) -> u64 {
    let mut c = window(true);
    desk::with(|d| (d.frame_times, d.covered, d.refresh_ns) = (true, true, 16_666_666));
    let start = 10_000;
    set_clock(start);
    c.req(PRESENTATION, 1, &[A::U(SURFACE), A::U(FEEDBACK)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 2, &[A::I(0), A::I(0), A::I(400), A::I(300)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let handed = desk::with(|d| d.shown);
    let mut now = start;
    loop {
        if now >= start + shown_after {
            desk::with(|d| d.shown = handed + 1);
        }
        set_clock(now);
        let next = c.s.tick();
        if to(&c.events(), FEEDBACK).iter().any(|e| e.opcode == PRESENTED) {
            return now - (start + shown_after);
        }
        now += next.unwrap_or(1).max(1);
        assert!(now < start + 500, "never told");
    }
}

/// The pixels between where the pointer is and where the client is told a
/// click landed.
fn click() -> i64 {
    let mut c = window(false);
    input::motion(60, 60);
    pump(&mut c.s.guest);
    c.events();
    input::motion(150, 150);
    pump(&mut c.s.guest);
    let there = to(&c.events(), POINTER).iter().find(|e| e.opcode == MOTION).map(|e| e.word(1));
    input::motion(60, 60);
    pump(&mut c.s.guest);
    c.events();
    c.s.guest.display.to_client.resize(1 << 20, 0);
    let held = c.s.guest.display.to_client.len();
    for x in 61..=150 {
        input::motion(x, x);
        pump(&mut c.s.guest);
    }
    input::post(input::Kind::ButtonDown, 0, 1, (150, 150), (0, 0));
    pump(&mut c.s.guest);
    c.s.guest.display.to_client.drain(..held);
    let evs = c.events();
    let mine = to(&evs, POINTER);
    let at = mine.iter().take_while(|e| e.opcode != BUTTON).filter(|e| e.opcode == MOTION).last();
    let told = at.map(|e| e.word(1)).or(Some(0x3c00)).map(|v| i64::from(v as i32) / 256);
    (there.map_or(0, |v| i64::from(v as i32) / 256) - told.unwrap_or(0)).abs()
}

trait Wh {
    fn into_wh(self) -> (u32, u32);
}

impl Wh for (u32, u32, u32) {
    fn into_wh(self) -> (u32, u32) {
        (self.0, self.1)
    }
}

fn main() {
    println!("hover: the serve loop asked to sleep {} ms in the input pass", hover());
    let told: Vec<u64> = (0..=16).map(feedback).collect();
    let mean = told.iter().sum::<u64>() as f64 / told.len() as f64;
    let worst = told.iter().max().copied().unwrap_or(0);
    println!(
        "feedback: told {mean:.1} ms after the frame is shown on average, {worst} ms at worst"
    );
    println!("click: lands {} px from the pointer", click());
}
