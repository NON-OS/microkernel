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

//! The trace's text: one header and one CSV row a frame, the same from the
//! capsule and the host harness.

use alloc::format;
use alloc::string::String;

use super::record::Stages;

pub const TAG: &str = "[FTRACE]";
pub const COLUMNS: &str =
    "frame,ready_ns,wake_ns,damage_ns,composite_ns,flip_ns,total_ns,rects,pixels";

pub fn header(clock: &str, per_ms: u64) -> String {
    format!("{TAG} # clock={clock} ticks_per_ms={per_ms}\n{TAG} {COLUMNS}\n")
}

pub fn ns(ticks: u64, per_ms: u64) -> u64 {
    if per_ms == 0 {
        return 0;
    }
    let n = ticks as u128 * 1_000_000 / per_ms as u128;
    u64::try_from(n).unwrap_or(u64::MAX)
}

/// One frame. An empty field is a stage the frame did not have.
pub fn row(frame: u32, s: &Stages, rects: u32, pixels: u64, per_ms: u64) -> String {
    let opt = |t: Option<u64>| t.map(|t| format!("{}", ns(t, per_ms))).unwrap_or_default();
    format!(
        "{TAG} {frame},{},{},{},{},{},{},{rects},{pixels}\n",
        opt(s.ready),
        opt(s.wake),
        ns(s.damage, per_ms),
        ns(s.composite, per_ms),
        ns(s.flip, per_ms),
        ns(s.total, per_ms),
    )
}

/// Under a frame's row: the plane scanned a client buffer out, and the
/// frame composed nothing but the band (its `pixels`).
pub fn direct(frame: u32, handle: u64) -> String {
    format!("{TAG} # direct frame={frame} handle={handle:#x}\n")
}
