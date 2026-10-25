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

//! The meter's two timeouts on the clock the capsule passes (monotonic,
//! mk_uptime_ms): a job step times out at exactly STEP_MS after it was
//! sent, and settle's report of a channel goes stale at exactly STALE_MS.

use nonos_nox_meter::timeout::{step_timed_out, still_fresh, STALE_MS, STEP_MS};

const T: u64 = 5_000;

#[test]
fn a_step_waits_until_one_ms_before_step_ms_and_times_out_at_it() {
    assert!(!step_timed_out(T, T), "just sent");
    assert!(!step_timed_out(T, T + STEP_MS - 1), "1 ms before");
    assert!(step_timed_out(T, T + STEP_MS), "at STEP_MS");
    assert!(step_timed_out(T, T + STEP_MS + 1), "past it");
}

#[test]
fn a_report_is_fresh_until_one_ms_before_stale_ms_and_stale_at_it() {
    assert!(still_fresh(T, T), "just reported");
    assert!(still_fresh(T, T + STALE_MS - 1), "1 ms before");
    assert!(!still_fresh(T, T + STALE_MS), "stale at STALE_MS");
    assert!(!still_fresh(T, T + STALE_MS + 1), "past it");
}

/// Why the clock must be monotonic: a `now` behind the stamp counts as no
/// time at all, so a wall clock stepped back would hold a step off ETIME,
/// and keep a channel fresh so it is not refreshed before a charge.
#[test]
fn a_now_behind_the_stamp_counts_as_no_time() {
    let late = STEP_MS * 3;
    assert!(!step_timed_out(late, 0));
    assert!(!step_timed_out(late, late - 1));
    assert!(step_timed_out(late, late + STEP_MS));
    assert!(still_fresh(late, 0));
    assert!(still_fresh(late, late - 1));
    assert!(!still_fresh(late, late + STALE_MS));
}

/// A stamp at either end of the clock neither wraps nor panics.
#[test]
fn the_ends_of_the_clock_do_not_wrap() {
    assert!(!step_timed_out(u64::MAX, u64::MAX));
    assert!(step_timed_out(0, u64::MAX));
    assert!(still_fresh(u64::MAX, 0));
    assert!(!still_fresh(0, u64::MAX));
}
