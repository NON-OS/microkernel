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

//! What a Linux app's first frame costs the Wayland bridge, on the host:
//! the shipped server (`crate::linux::wayland`) taking a Qt-shaped client
//! from its connection to its first frame on a 1920x1080 desktop, each
//! stage timed from the client's first request of it to the bridge's last
//! answer.
//!
//! - globals: wl_display.get_registry, every global the bridge offers
//!   bound as Qt 6 binds them, and the keyboard asked for (its keymap made
//!   and handed over).
//! - configure: a surface made an xdg toplevel with a title and an app
//!   id, its first commit with no buffer, up to the configure.
//! - first frame: the configure acked, a W by H shm pool and buffer over a
//!   memfd painted beforehand, attached, damaged whole and committed, up
//!   to the layer on the desktop.
//!
//! Each line gives the median and the 99th over the runs, and the calls
//! the bridge made on the desktop (compositor and window manager).
//!
//!     cargo +nightly-2026-01-16 run --release --example startup_bench [W H]

use std::time::Instant;

use nonos_app_skeleton::desk;
use nonos_capsule_wayland_proofs::client::*;

const RUNS: usize = 200;
const POINTER: u32 = 14;

/// The globals a Qt 6 client binds, at the versions it asks, and the ids
/// this bench gives them.
const QT: [(&str, u32, u32); 15] = [
    ("wl_compositor", 6, COMPOSITOR),
    ("xdg_wm_base", 6, WM_BASE),
    ("wl_seat", 9, SEAT),
    ("wl_shm", 1, SHM),
    ("wl_output", 4, 30),
    ("wl_subcompositor", 1, 31),
    ("wl_data_device_manager", 3, 32),
    ("zxdg_decoration_manager_v1", 1, 33),
    ("zxdg_output_manager_v1", 3, 34),
    ("wp_viewporter", 1, 35),
    ("xdg_activation_v1", 1, 36),
    ("zwp_primary_selection_device_manager_v1", 1, 37),
    ("wp_cursor_shape_manager_v1", 1, 38),
    ("wp_fractional_scale_manager_v1", 1, 39),
    ("wp_presentation", 1, 40),
];

/// A memfd of `w` by `h` pixels, painted: the client's own drawing, done
/// before the stage it is shown in.
fn painted(w: i32, h: i32) -> i32 {
    let px: Vec<u8> = (0..w * h).flat_map(|_| 0xFF20_40A0u32.to_le_bytes()).collect();
    // SAFETY: a new memfd, sized and written, handed to the session after.
    unsafe {
        let fd = libc::memfd_create(c"client".as_ptr(), libc::MFD_CLOEXEC);
        assert!(fd >= 0 && libc::ftruncate(fd, px.len() as libc::off_t) == 0);
        assert_eq!(libc::write(fd, px.as_ptr().cast(), px.len()), px.len() as isize);
        fd
    }
}

/// One app's start: microseconds for each stage, and the desktop's calls.
fn start(w: i32, h: i32) -> ([f64; 3], usize) {
    desk::with(|d| {
        *d = desk::Desk::default();
        (d.width, d.height) = (1920, 1080);
    });
    let fd = painted(w, h);
    let mut c = Client::new();
    let t = Instant::now();
    c.bind(REGISTRY, &QT);
    c.req(SEAT, 1, &[A::U(KEYBOARD)]);
    c.req(SEAT, 0, &[A::U(POINTER)]);
    c.events();
    let globals = t.elapsed();
    let t = Instant::now();
    c.req(COMPOSITOR, 0, &[A::U(SURFACE)]);
    c.req(WM_BASE, 2, &[A::U(XDG_SURFACE), A::U(SURFACE)]);
    c.req(XDG_SURFACE, 1, &[A::U(TOPLEVEL)]);
    c.req(TOPLEVEL, 2, &[A::S("Telegram")]);
    c.req(TOPLEVEL, 3, &[A::S("org.telegram.desktop")]);
    c.req(SURFACE, 6, &[]);
    let configure = to(&c.events(), XDG_SURFACE).last().map(|e| e.word(0)).expect("configured");
    let configured = t.elapsed();
    let t = Instant::now();
    c.req(XDG_SURFACE, 4, &[A::U(configure)]);
    c.req_fds(SHM, 0, &[A::U(POOL), A::I(w * h * 4)], &[fd]);
    c.req(POOL, 0, &[A::U(BUFFER), A::I(0), A::I(w), A::I(h), A::I(w * 4), A::U(0)]);
    c.req(SURFACE, 1, &[A::U(BUFFER), A::I(0), A::I(0)]);
    c.req(SURFACE, 9, &[A::I(0), A::I(0), A::I(w), A::I(h)]);
    c.req(SURFACE, 6, &[]);
    c.events();
    let first = t.elapsed();
    let shown = desk::with(|d| !d.layers.is_empty());
    assert!(shown, "the first commit put a layer on the desktop");
    let calls = desk::with(|d| d.calls.len());
    let us = |d: std::time::Duration| d.as_secs_f64() * 1e6;
    ([us(globals), us(configured), us(first)], calls)
}

fn pct(v: &mut [f64], p: f64) -> f64 {
    v.sort_by(f64::total_cmp);
    v[((v.len() - 1) as f64 * p).round() as usize]
}

fn main() {
    let args: Vec<i32> = std::env::args().skip(1).filter_map(|a| a.parse().ok()).collect();
    let (w, h) = match args[..] {
        [w, h] if w > 0 && h > 0 => (w, h),
        _ => (1280, 720),
    };
    let mut stages: [Vec<f64>; 4] = Default::default();
    let mut calls = 0;
    for _ in 0..RUNS {
        let (t, n) = start(w, h);
        for (i, us) in t.iter().enumerate() {
            stages[i].push(*us);
        }
        stages[3].push(t.iter().sum());
        calls = n;
    }
    println!("startup_bench: a {w}x{h} Qt-shaped window on 1920x1080, {RUNS} runs, p50 / p99 us");
    for (name, v) in ["globals", "configure", "first frame", "connect to first frame"]
        .into_iter()
        .zip(stages.iter_mut())
    {
        println!("  {name:<23} {:>9.1} / {:>9.1}", pct(v, 0.5), pct(v, 0.99));
    }
    println!("  desktop calls to the first frame: {calls}");
}
