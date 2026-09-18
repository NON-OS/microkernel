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

use nonos_bench_core::{sort, Summary};

/// What one timing run produced, in counter ticks.
///
/// Percentiles, not a minimum and a mean. The old shape quoted `min` as "the
/// cost of the work itself" and it is a defensible number, but it is the number
/// that cannot regress: a change that makes one call in fifty take twenty times
/// as long leaves `min` untouched and `avg` barely moved, and that call is the
/// one a person notices. The tail is the measurement.
///
/// The reduction is `nonos_bench_core`, the same code the `bench` command in the
/// terminal runs. Two harnesses reporting different statistics about one machine
/// is how a regression hides between them.
pub struct Sample {
    pub summary: Summary,
}

impl Sample {
    /// Fold a set of tick counts into the figures worth printing.
    ///
    /// Sorts in place: the caller's buffer is scratch that is consumed here, and
    /// a kernel path has nowhere to copy a second one to.
    pub(crate) fn from_runs(runs: &mut [u64]) -> Self {
        sort(runs);
        // Overhead is not subtracted here. The kernel harness times whole
        // operations that dwarf a counter read, and claiming a correction that
        // was never calibrated would be worse than leaving it in and saying so.
        Sample { summary: Summary::from_sorted(runs, 0) }
    }

    /// The median in nanoseconds, or `None` where the platform never reported a
    /// counter frequency and the conversion would be invented.
    pub fn p50_nanos(&self) -> Option<u64> {
        let hz = crate::arch::time_counter_hz();
        if hz == 0 {
            return None;
        }
        Some(((self.summary.p50 as u128 * 1_000_000_000u128) / hz as u128) as u64)
    }
}
