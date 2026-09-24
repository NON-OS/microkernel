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
use crate::sys::serial;

/// Print one measurement on the same channel the boot markers use, so the
/// benchmark harness parses both with one reader.
///
/// Shape is `[BENCH] <uptime> micro:<name> iters p50 p95 p99 max` in ticks, and
/// nanoseconds only where the platform actually told us the counter rate. Where
/// it did not, the field is absent rather than estimated: a made up nanosecond
/// figure would be quoted later as if it had been measured.
pub fn report(name: &[u8], sample: &Sample) {
    serial::print(b"[BENCH] ");
    serial::print_dec(crate::sys::timer::uptime_ms());
    serial::print(b" micro:");
    serial::print(name);
    serial::print(b" iters=");
    serial::print_dec(sample.summary.samples as u64);
    serial::print(b" p50=");
    serial::print_dec(sample.summary.p50);
    serial::print(b" p95=");
    serial::print_dec(sample.summary.p95);
    serial::print(b" p99=");
    serial::print_dec(sample.summary.p99);
    serial::print(b" max=");
    serial::print_dec(sample.summary.max);

    match sample.p50_nanos() {
        Some(ns) => {
            serial::print(b" p50_ns=");
            serial::print_dec(ns);
            serial::println(b"");
        }
        None => serial::println(b" min_ns=uncalibrated"),
    }
}
