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

use super::sample::Sample;
use crate::sys::serial::Line;

/// Print one measurement on the same channel the boot markers use, so the
/// benchmark harness parses both with one reader.
///
/// Shape is `[BENCH] <uptime> micro:<name> iters p50 p95 p99 max` in ticks, and
/// nanoseconds only where the platform actually told us the counter rate. Where
/// it did not, the field is absent rather than estimated: a made up nanosecond
/// figure would be quoted later as if it had been measured.
pub fn report(name: &[u8], sample: &Sample) {
    let s = &sample.summary;
    // One line, written once: no other CPU's line lands inside it.
    let mut l = Line::new();
    l.str(b"[BENCH] ").dec(crate::sys::timer::uptime_ms()).str(b" micro:").str(name);
    l.str(b" iters=").dec(s.samples as u64).str(b" p50=").dec(s.p50).str(b" p95=").dec(s.p95);
    l.str(b" p99=").dec(s.p99).str(b" max=").dec(s.max);
    match sample.p50_nanos() {
        Some(ns) => l.str(b" p50_ns=").dec(ns),
        None => l.str(b" min_ns=uncalibrated"),
    };
    l.end();
}
