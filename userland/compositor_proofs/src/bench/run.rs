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

//! Runs a scene and prints it as the capsule prints a trace.

use crate::frame_pacer::trace::line;
use crate::frame_pacer::trace::record::{FrameRecord, Stages};

use super::clock::{Clock, PER_MS};
use super::scenes::{desk_for, step, Kind};

/// `frames` frames of `kind` at `width` by `height`, after one full frame
/// that is not counted.
pub fn run(kind: Kind, width: u32, height: u32, frames: u32) -> Vec<FrameRecord> {
    let clock = Clock::new();
    let mut d = desk_for(kind, width, height);
    d.frame(&clock);
    (0..frames)
        .map(|i| {
            step(&mut d, kind, i);
            d.frame(&clock)
        })
        .collect()
}

pub fn csv(records: &[FrameRecord]) -> String {
    let mut out = line::header("host-ns", PER_MS);
    for (i, r) in records.iter().enumerate() {
        out += &line::row(i as u32, &r.stages(PER_MS), r.rects, r.pixels, PER_MS);
    }
    out
}

/// The median and the 99th percentile of one stage, in nanoseconds.
pub fn spread(records: &[FrameRecord], stage: fn(&Stages) -> u64) -> (u64, u64) {
    let mut v: Vec<u64> = records.iter().map(|r| stage(&r.stages(PER_MS))).collect();
    if v.is_empty() {
        return (0, 0);
    }
    v.sort_unstable();
    let at = |q: usize| v[(v.len() - 1) * q / 100];
    (at(50), at(99))
}
