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

/// Emit one case as a marker the host side can collect.
///
/// Ticks are reported alongside nanoseconds because the tick rate is a
/// property of the part, and a reader comparing two machines needs the raw
/// count as well as the conversion.
pub fn report(name: &[u8], sample: &mut Sample, overhead: u64) {
    sample.sort();
    let hz = crate::arch::time_counter_hz();
    let net = |ticks: u64| ticks.saturating_sub(overhead);
    let ns = |ticks: u64| {
        if hz == 0 {
            0
        } else {
            net(ticks).saturating_mul(1_000_000_000) / hz
        }
    };

    let mut l = crate::sys::serial::Line::new();
    l.str(b"[UBENCH] ").str(name).str(b" n=").dec(sample.len() as u64).str(b" min=");
    l.dec(net(sample.min())).str(b" p50=").dec(net(sample.quantile(1, 2))).str(b" p95=");
    l.dec(net(sample.quantile(95, 100))).str(b" max=").dec(net(sample.max()));
    l.str(b" ticks p50_ns=").dec(ns(sample.quantile(1, 2))).end();
}
