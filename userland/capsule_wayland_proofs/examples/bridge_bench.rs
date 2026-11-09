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

//! What a guest's frame costs the Wayland bridge, on the host: the shipped
//! server (`crate::linux::wayland`) serving a client that draws a `W` by `H`
//! window into two buffers in turn, as a double-buffered client does, each
//! commit timed from its request to its answer. A whole-window commit and
//! the damage of a scroll (a full-width list under a toolbar, and a chat
//! pane beside a list of chats), of a drag inside the window (a 256x256
//! piece moved 8 pixels a frame), a 120x40 button, a 256x64 box, and a
//! terminal's selection drag, its selection's rows and its last row said
//! as two boxes far apart, framed (the NØNOS frame round it), full screen
//! (unframed) and in its own frame with the shadow it draws round it (a
//! client that binds xdg-decoration and makes no decoration, its window
//! geometry inside an 11 px shadow, as Telegram's Qt window). Each line gives
//! what a commit read from the guest as the capsule reads it: kernel peer
//! calls, KiB, and guest pages the kernel walks. The bridge's own trace
//! line is printed after.
//!
//!     cargo +nightly-2026-01-16 run --release --example bridge_bench [W H]

use std::time::Instant;

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;
use nonos_capsule_wayland_proofs::linux::guest_reads::{self, Reads};

const FRAMES: usize = 512;
const RUNS: usize = 7;
/// The shadow a client that draws its own frame draws round its window.
const SHADE: i32 = 11;
const DECO_REGISTRY: u32 = 40;
const DECO_MANAGER: u32 = 41;

/// How the bench window is shown.
#[derive(Clone, Copy)]
enum Kind {
    Framed,
    FullScreen,
    OwnShadow,
}

impl Kind {
    fn name(self) -> &'static str {
        match self {
            Kind::Framed => "framed",
            Kind::FullScreen => "full screen",
            Kind::OwnShadow => "own frame and shadow",
        }
    }

    /// Where the window's pixels start in its buffer.
    fn inset(self) -> i32 {
        match self {
            Kind::OwnShadow => SHADE,
            _ => 0,
        }
    }
}

fn window(w: i32, h: i32, kind: Kind) -> Client {
    let mut c = toplevel(6);
    c.req(TOPLEVEL, 2, &[A::S("bench")]);
    match kind {
        Kind::Framed => {}
        Kind::FullScreen => c.req(TOPLEVEL, 11, &[A::U(0)]),
        Kind::OwnShadow => {
            c.bind(DECO_REGISTRY, &[("zxdg_decoration_manager_v1", 1, DECO_MANAGER)]);
            c.req(XDG_SURFACE, 3, &[A::I(SHADE), A::I(SHADE), A::I(w), A::I(h)]);
        }
    }
    c.req(SURFACE, 6, &[]);
    let configure = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    c.req(XDG_SURFACE, 4, &[A::U(configure)]);
    let whole = (w + 2 * kind.inset(), h + 2 * kind.inset());
    for n in 0..2 {
        c.painted(SHM, 100 + 2 * n, 101 + 2 * n, whole, 0xFF20_40A0 + n);
    }
    c.req(SURFACE, 1, &[A::U(101), A::I(0), A::I(0)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    c
}

type Box4 = (i32, i32, i32, i32);

/// The median and the 99th of `FRAMES` commits' times, in microseconds,
/// commit `i` damaging each box of `b(i)`, and what they read from the
/// guest. The boxes are the window's, at `inset` in the buffer.
fn run(c: &mut Client, b: &dyn Fn(usize) -> Vec<Box4>, inset: i32) -> (f64, f64, Reads) {
    let mut t = Vec::with_capacity(FRAMES);
    guest_reads::take();
    for i in 0..FRAMES {
        let buffer = 101 + 2 * (i as u32 % 2);
        c.req(SURFACE, 1, &[A::U(buffer), A::I(0), A::I(0)]);
        for b in b(i) {
            c.req(SURFACE, 9, &[A::I(b.0 + inset), A::I(b.1 + inset), A::I(b.2), A::I(b.3)]);
        }
        let at = Instant::now();
        c.req(SURFACE, 6, &[]);
        t.push(at.elapsed().as_secs_f64() * 1e6);
        c.events();
    }
    t.sort_by(f64::total_cmp);
    (t[FRAMES / 2], t[FRAMES * 99 / 100], guest_reads::take())
}

/// The damage each kind of frame says, for a `w` by `h` window.
type Frame = (&'static str, Box<dyn Fn(usize) -> Vec<Box4>>);

fn frames(w: i32, h: i32) -> Vec<Frame> {
    vec![
        ("whole", Box::new(move |_| vec![(0, 0, w, h)])),
        ("scroll, full width", Box::new(move |_| vec![(0, 48, w, h - 48)])),
        ("scroll, chat pane", Box::new(move |_| vec![(w / 4, 48, w - w / 4, h - 48)])),
        ("drag in window", Box::new(move |i| vec![((8 * i as i32) % (w - 264), h / 3, 264, 256)])),
        ("120x40 button", Box::new(move |_| vec![(w / 2, h / 2, 120, 40)])),
        ("256x64 box", Box::new(|_| vec![(64, 64, 256, 64)])),
        (
            "selection and last row",
            Box::new(move |i| vec![(0, 64 + 17 * (i as i32 % 8), w, 34), (0, h - 34, w, 34)]),
        ),
    ]
}

fn main() {
    let args: Vec<i32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let (w, h) = match args[..] {
        [w, h] => (w, h),
        _ => (1920, 1080),
    };
    // A display the framed window fits on whole.
    desk::with(|d| (d.width, d.height) = (w as u32 + 256, h as u32 + 256));
    // Copied out of the guest, as before MkPeerPin, then shown in place.
    for (how, pins) in [("copied", false), ("pinned", true)] {
        nonos_libc::world::with(|w| w.allow_pins = pins);
        bench(w, h, how);
    }
}

fn bench(w: i32, h: i32, how: &str) {
    for kind in [Kind::Framed, Kind::FullScreen, Kind::OwnShadow] {
        let name = kind.name();
        for (what, b) in frames(w, h) {
            let (mut best, mut read) = ((f64::MAX, f64::MAX), Reads::default());
            for _ in 0..RUNS {
                let mut c = window(w, h, kind);
                let (p50, p99, r) = run(&mut c, &*b, kind.inset());
                (best, read) = ((best.0.min(p50), best.1.min(p99)), r);
            }
            let n = FRAMES as u64;
            let (calls, kib, pages) = (read.calls / n, read.bytes / 1024 / n, read.pages / n);
            println!(
                "{w}x{h} {how}, {name}, {what}: p50 {:.1} us, p99 {:.1} us a commit",
                best.0, best.1
            );
            println!("  read a commit: {calls} peer calls, {kib} KiB, {pages} guest pages");
            print!("  trace: {}", last_trace());
        }
    }
}

/// The bridge's latest line on its frames (commit/launch_clock.rs).
fn last_trace() -> String {
    let said = nonos_libc::world::with(|w| w.said.clone());
    let line = said.iter().rev().find(|l| l.windows(8).any(|w| w == b" frames,"));
    line.map_or("none\n".into(), |l| String::from_utf8_lossy(l).into_owned())
}
