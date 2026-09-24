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

//! What a run of measurements is reduced to.
//!
//! Percentiles and a maximum, never a mean. An average over a thousand IPC
//! round trips tells you what the common case costs and hides the one in a
//! hundred that took forty times as long, which is the one a person actually
//! notices and the one a missed wake shows up as.
//!
//! Nearest-rank, so every value reported is a value that was measured. An
//! interpolating percentile invents a number that never occurred, which is a
//! strange thing to publish as evidence.

/// A finished measurement, in cycles, overhead already subtracted.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Summary {
    pub samples: u32,
    pub p50: u64,
    pub p95: u64,
    pub p99: u64,
    pub max: u64,
    /// The counter overhead taken off every sample, kept so a reader can add it
    /// back rather than having to trust that it was reasonable.
    pub overhead: u64,
}

impl Summary {
    /// Reduce a sorted slice. Sorting is the caller's job because the buffer is
    /// theirs and this crate allocates nothing.
    ///
    /// An empty run reports zeros rather than refusing: a benchmark that could
    /// not take a sample has a real answer, and it is not an error state that
    /// callers should have to unwrap around.
    pub fn from_sorted(sorted: &[u64], overhead: u64) -> Self {
        if sorted.is_empty() {
            return Summary { overhead, ..Summary::default() };
        }
        Summary {
            samples: sorted.len() as u32,
            p50: nearest_rank(sorted, 50),
            p95: nearest_rank(sorted, 95),
            p99: nearest_rank(sorted, 99),
            max: sorted[sorted.len() - 1],
            overhead,
        }
    }
}

/// The smallest value at or above `pct` of the run, by nearest rank.
///
/// Index `ceil(pct * n / 100) - 1`, computed in integers so it cannot drift.
/// Clamped at the top so p99 of a two-sample run is the larger of the two
/// rather than an out-of-bounds panic, which is the shape a short run takes.
fn nearest_rank(sorted: &[u64], pct: u64) -> u64 {
    let n = sorted.len() as u64;
    let rank = (pct * n).div_ceil(100).max(1);
    sorted[(rank - 1).min(n - 1) as usize]
}
