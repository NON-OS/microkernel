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

//! The frame trace on the host: every scripted scene at 1920x1080 and
//! 3840x2160 through the compositor's own compose path.
//!
//!     cargo run --release --example frame_trace -- <out dir> [frames]
//!
//! Writes one CSV a scene and size to <out dir>, in the capsule's format,
//! and prints the median and 99th percentile of each stage.

use compositor_proofs::bench::run::{csv, run, spread};
use compositor_proofs::bench::scenes::Kind;

const SIZES: [(u32, u32); 2] = [(1920, 1080), (3840, 2160)];

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let out = std::path::PathBuf::from(args.next().unwrap_or_else(|| ".".into()));
    let frames = args.next().and_then(|n| n.parse().ok()).unwrap_or(240);
    std::fs::create_dir_all(&out)?;
    println!("| scene | size | rects/frame | px/frame | compose p50 / p99 us | flip p50 / p99 us | frame p50 / p99 us |");
    println!("|---|---|---|---|---|---|---|");
    for (w, h) in SIZES {
        for kind in Kind::ALL {
            let recs = run(kind, w, h, frames);
            std::fs::write(out.join(format!("{}-{w}x{h}.csv", kind.name())), csv(&recs))?;
            let n = recs.len().max(1) as u64;
            let rects = recs.iter().map(|r| r.rects as u64).sum::<u64>() / n;
            let px = recs.iter().map(|r| r.pixels).sum::<u64>() / n;
            let us = |(a, b): (u64, u64)| format!("{:.1} / {:.1}", a as f64 / 1e3, b as f64 / 1e3);
            println!(
                "| {} | {w}x{h} | {rects} | {px} | {} | {} | {} |",
                kind.name(),
                us(spread(&recs, |s| s.composite)),
                us(spread(&recs, |s| s.flip)),
                us(spread(&recs, |s| s.total)),
            );
        }
    }
    Ok(())
}
