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

//! The meter's two timeouts, judged on the monotonic clock the
//! capsule passes as `now_ms` and takes each stamp from (mk_uptime_ms): a
//! step of the wall clock neither fires one early nor holds it off. Both
//! stamps live in memory only, so uptime starting again at boot loses
//! nothing. A `now` behind its stamp counts as no time passed.

/// A job step with no answer in this long fails its job ETIME (v1.4.15).
pub const STEP_MS: u64 = 150_000;
/// A channel settle reported this long ago is refreshed before a charge.
pub const STALE_MS: u64 = 60_000;

/// Whether the step sent at `sent_ms` has waited STEP_MS by `now_ms`.
pub fn step_timed_out(sent_ms: u64, now_ms: u64) -> bool {
    now_ms.saturating_sub(sent_ms) >= STEP_MS
}

/// Whether settle's report at `seen_ms` is younger than STALE_MS at `now_ms`.
pub fn still_fresh(seen_ms: u64, now_ms: u64) -> bool {
    now_ms.saturating_sub(seen_ms) < STALE_MS
}
