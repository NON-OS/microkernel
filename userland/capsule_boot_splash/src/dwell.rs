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

//! When the splash steps aside. It lingers until the desktop shell registers
//! (the desktop is coming up), then a short settle so the desktop paints
//! behind this overlay before the hand-off. MAX_DWELL is a hard cap so the
//! splash never hangs if the shell fails to appear. Time is the monotonic
//! uptime, not the wall clock: mk_time_millis returns -61 until the wall clock
//! has an epoch anchor, which would pin both gates shut on a boot that has
//! not synced one yet.

const SETTLE_MS: i64 = 1000;
const MAX_DWELL_MS: i64 = 30_000;
pub(crate) const MAX_ITERS: u32 = 8_000_000;

/// `elapsed` since the splash went up; `desktop_up_at` when the shell was
/// first seen (negative: not yet). The detail view stays until dismissed.
pub(crate) fn over(elapsed: i64, now: i64, desktop_up_at: i64, detail: bool, iters: u32) -> bool {
    let handed_off = desktop_up_at >= 0 && now >= desktop_up_at && now - desktop_up_at >= SETTLE_MS;
    (!detail && (handed_off || elapsed >= MAX_DWELL_MS)) || iters >= MAX_ITERS
}
