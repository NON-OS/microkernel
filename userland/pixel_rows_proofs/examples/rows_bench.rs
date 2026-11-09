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

//! The Wayland bridge's pixel work before and after, on the host:
//!
//!     cargo run --release --example rows_bench
//!
//! Each line is the median of 15 timed runs of one operation over a whole
//! frame (or a damaged box), old per-pixel routine against the rows.

use std::time::Instant;

use pixel_rows_proofs::level::level;
use pixel_rows_proofs::nearest::{nearest_row, NearestCols};
use pixel_rows_proofs::reference::{lay_box_row, lay_row, make_opaque, paint_row, resample};
use pixel_rows_proofs::rng::Rng;
use pixel_rows_proofs::{blend, opaque, words};

const GROUND: u32 = 0xFF20_2228;

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

/// A frame of pixels, `translucent` of each 16 not opaque.
fn frame(r: &mut Rng, n: usize, translucent: u64) -> Vec<u8> {
    let mut v = r.pixels(n);
    for (i, p) in v.chunks_exact_mut(4).enumerate() {
        if (i as u64 * 7 + r.draw()) % 16 >= translucent {
            p[3] = 0xFF;
        }
    }
    v
}

fn line(what: &str, size: &str, old: f64, new: f64) {
    println!("| {what} | {size} | {old:.3} | {new:.3} | {:.1}x |", old / new);
}

fn main() {
    println!("rows path on this host: {:?}", level());
    println!("| operation | area | old ms | rows ms | speed-up |");
    println!("|---|---|---|---|---|");
    let mut r = Rng::new(1);
    for (w, h) in [(1920usize, 1080usize), (3840, 2160)] {
        let size = format!("{w}x{h}");
        let n = w * h;
        let src = frame(&mut r, n, 0);
        // What Guest::read_into does per commit: the client's rows copied
        // out of its pool into the bridge's own buffer, whole or a box.
        let mut out = vec![0u8; n * 4];
        let whole = median(|| out.copy_from_slice(&src));
        let boxed = median(|| {
            for y in 0..64 {
                let at = (y + 100) * w * 4 + 400 * 4;
                out[at..at + 256 * 4].copy_from_slice(&src[at..at + 256 * 4]);
            }
        });
        println!("| guest read (a copy), whole frame | {size} | {whole:.3} | - | - |");
        println!("| guest read (a copy), 256x64 box | {size} | {boxed:.3} | - | - |");
        let mut buf = src.clone();
        let old = median(|| make_opaque(&mut buf, w as u32, h as u32, (w * 4) as u32));
        let new = median(|| buf.chunks_exact_mut(w * 4).for_each(opaque::opaque_row));
        line("XRGB to ARGB", &size, old, new);

        for (label, t) in [("paint, opaque client", 0), ("paint, 1/4 translucent", 4)] {
            let client = frame(&mut r, n, t);
            let mut out = vec![0u32; n];
            let old = median(|| {
                for (row, dst) in client.chunks_exact(w * 4).zip(out.chunks_exact_mut(w)) {
                    paint_row(dst, row, GROUND);
                }
            });
            let new = median(|| {
                let bytes = words::bytes_mut(&mut out);
                for (row, dst) in client.chunks_exact(w * 4).zip(bytes.chunks_exact_mut(w * 4)) {
                    blend::over_colour(dst, row, GROUND);
                }
            });
            line(label, &size, old, new);
        }

        let layer = frame(&mut r, n, 8);
        let mut base = frame(&mut r, n, 0);
        let old = median(|| {
            for (d, s) in base.chunks_exact_mut(w * 4).zip(layer.chunks_exact(w * 4)) {
                lay_row(d, s, true);
            }
        });
        let new = median(|| {
            for (d, s) in base.chunks_exact_mut(w * 4).zip(layer.chunks_exact(w * 4)) {
                blend::over_row(d, s);
            }
        });
        line("subsurface over, 1/2 translucent", &size, old, new);
        // The same on the SSE2 path, which every x86-64 runs.
        let sse2 = median(|| {
            for (d, s) in base.chunks_exact_mut(w * 4).zip(layer.chunks_exact(w * 4)) {
                let k = d.len() / 16 * 16;
                // SAFETY: SSE2 is in every x86-64; both rows hold `k` bytes.
                unsafe {
                    pixel_rows_proofs::sse2::blend::<{ blend::SRC_OVER }>(
                        d.as_mut_ptr(),
                        s.as_ptr(),
                        k,
                        0,
                    )
                };
            }
        });
        line("subsurface over, SSE2 path", &size, old, sse2);

        // A damaged box: a 256x64 strip of text redrawn.
        let (bw, bh) = (256usize, 64usize);
        let old = median(|| {
            for y in 0..bh {
                let at = (y + 100) * w * 4 + 400 * 4;
                lay_box_row(&mut base[at..at + bw * 4], &layer[at..at + bw * 4], GROUND);
            }
        });
        let new = median(|| {
            for y in 0..bh {
                let at = (y + 100) * w * 4 + 400 * 4;
                let (d, s) = (&mut base[at..at + bw * 4], &layer[at..at + bw * 4]);
                blend::over_colour(d, s, GROUND);
            }
        });
        line("damage box 256x64 onto ground", &size, old, new);

        // Shown at one and a half times: a 2/3-size buffer resampled.
        let (sw, sh) = (w * 2 / 3, h * 2 / 3);
        let small = frame(&mut r, sw * sh, 0);
        let mut big = vec![0u8; n * 4];
        let shape = (sw as u32, sh as u32, (sw * 4) as u32);
        let old = median(|| {
            resample(&small, shape, &mut big, (w as u32, h as u32));
        });
        let new = median(|| {
            let Some(cols) = NearestCols::new(sw as u32, w as u32) else { return };
            for (y, out) in big.chunks_exact_mut(w * 4).enumerate() {
                let row = ((2 * y + 1) * sh / (2 * h)).min(sh - 1) * sw * 4;
                nearest_row(out, &small[row..row + sw * 4], &cols, 0);
            }
        });
        line("nearest scale x1.5", &size, old, new);
    }
}
