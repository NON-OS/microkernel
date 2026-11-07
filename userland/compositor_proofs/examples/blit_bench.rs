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

//! The software blitter before and after, on the host:
//!
//!     cargo run --release --example blit_bench
//!
//! Each line is the median of 15 runs over a whole frame: the routine as it
//! stood before the change (bench/reference.rs) against the shipping one.

use std::time::Instant;

use compositor_proofs::bench::reference::{self, surface};
use compositor_proofs::damage::Rect;
use compositor_proofs::sw_blitter::{row::composite_row, scaled, upscale};

fn median(mut f: impl FnMut()) -> f64 {
    f();
    let mut t: Vec<f64> = (0..15)
        .map(|_| {
            let s = Instant::now();
            f();
            s.elapsed().as_secs_f64() * 1e3
        })
        .collect();
    t.sort_by(|a, b| a.total_cmp(b));
    t[7]
}

fn line(what: &str, size: &str, old: f64, new: f64) {
    println!("| {what} | {size} | {old:.3} | {new:.3} | {:.1}x |", old / new);
}

fn pixels(n: usize, seed: u32, alpha: impl Fn(usize) -> u32) -> Vec<u32> {
    let mut x = seed | 1;
    (0..n)
        .map(|i| {
            x ^= x << 13;
            x ^= x >> 17;
            x ^= x << 5;
            (x & 0x00FF_FFFF) | alpha(i) << 24
        })
        .collect()
}

fn main() {
    println!("| operation | frame | old ms | new ms | speed-up |");
    println!("|---|---|---|---|---|");
    for (w, h) in [(1920u32, 1080u32), (3840, 2160)] {
        let size = format!("{w}x{h}");
        let n = (w * h) as usize;
        let mut dst = pixels(n, 3, |_| 0xFF);
        for (what, alpha) in [
            ("layer, opaque", (|_| 0xFF) as fn(usize) -> u32),
            ("layer, translucent (shadow, dock)", |i| (i as u32 * 7) & 0xFF),
            (
                "layer, 1/2 clear 1/2 opaque in runs of 16",
                |i| if i / 16 % 2 == 0 { 0 } else { 0xFF },
            ),
        ] {
            let src = pixels(n, 5, alpha);
            let rows = |f: &dyn Fn(&[u32], &mut [u32]), dst: &mut Vec<u32>| {
                for (s, d) in src.chunks_exact(w as usize).zip(dst.chunks_exact_mut(w as usize)) {
                    f(s, d);
                }
            };
            let old = median(|| rows(&reference::composite_row, &mut dst));
            let new = median(|| rows(&composite_row, &mut dst));
            line(what, &size, old, new);
        }
    }
    // The canvas enlarged onto the panel: 2x (q 8) and 1.5x (q 6).
    for (cw, ch, q) in [(1920u32, 1080u32, 8u32), (1920, 1080, 6), (2560, 1440, 6)] {
        let (sw, sh) = (cw * q / 4, ch * q / 4);
        let mut canvas = pixels((cw * ch) as usize, 9, |_| 0xFF);
        let mut screen = vec![0u32; (sw * sh) as usize];
        let rect = Rect { x: 0, y: 0, width: cw, height: ch };
        let c = surface(&mut canvas, cw, ch);
        let s = surface(&mut screen, sw, sh);
        let (old, new) = if q % 4 == 0 {
            (
                median(|| {
                    let _ = reference::upscale(c, s, rect, q / 4);
                }),
                median(|| {
                    let _ = upscale(c, s, rect, q / 4);
                }),
            )
        } else {
            (
                median(|| {
                    let _ = reference::stretch(c, s, rect, q);
                }),
                median(|| {
                    let _ = scaled(c, s, rect, q);
                }),
            )
        };
        line(
            &format!("canvas {cw}x{ch} shown x{}", q as f64 / 4.0),
            &format!("{sw}x{sh}"),
            old,
            new,
        );
    }
}
