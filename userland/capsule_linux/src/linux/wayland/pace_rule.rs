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

//! When a surface's frame callbacks are answered: at most once a display
//! refresh, on a grid of the refresh the compositor reports (OP_FRAME_TIME's
//! refresh period, output.rs), 60 a second when it reports none. A slot is
//! one period in whole milliseconds, 16 or 17 at 60 Hz, 6 or 7 at 144, so a
//! client drawing at once is answered once a refresh, never twice in one
//! slot. Pure, so the host proofs hold it.

/// The refresh period when the compositor reports none: a sixtieth of a
/// second, in nanoseconds.
pub const DEFAULT_PERIOD_NS: u64 = 1_000_000_000 / 60;

/// The periods believed: 10 Hz to 500 Hz. Outside them a report is taken
/// for none.
const PERIODS_NS: core::ops::RangeInclusive<u64> = 2_000_000..=100_000_000;

/// The period frames are paced at, given the compositor's report (0 for
/// none).
pub fn period(reported_ns: u64) -> u64 {
    if PERIODS_NS.contains(&reported_ns) {
        reported_ns
    } else {
        DEFAULT_PERIOD_NS
    }
}

/// The refresh wl_output's mode says, in millihertz, for `period_ns`.
pub fn refresh_mhz(period_ns: u64) -> u32 {
    (1_000_000_000_000 / period_ns.max(1)) as u32
}

/// The first slot boundary after `at`, in milliseconds.
fn next_slot(at: u64, period_ns: u64) -> u64 {
    let k = at.saturating_mul(1_000_000) / period_ns + 1;
    k.saturating_mul(period_ns).div_ceil(1_000_000)
}

/// When callbacks owed at `now` may be answered, the surface's last answer
/// having gone at `last`: at once for a surface idle a slot or more, or
/// never answered, else at the next slot after its last.
pub fn due(last: Option<u64>, now: u64, period_ns: u64) -> u64 {
    last.map_or(now, |l| next_slot(l, period_ns).max(now))
}

/// How long until the first surface that holds callbacks is due, each
/// given by its last answer; None when none holds any.
pub fn next(holding: impl Iterator<Item = Option<u64>>, now: u64, period_ns: u64) -> Option<u64> {
    holding.map(|last| due(last, now, period_ns) - now).min()
}
