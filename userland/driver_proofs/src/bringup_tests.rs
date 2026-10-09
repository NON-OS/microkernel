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

//! The driver bring-up schedule: bounded, backing off, and never a spin.

use crate::bringup_policy::{
    delay_after, next, total_sleep_ms, Next, BRINGUP_ATTEMPTS, BRINGUP_FIRST_DELAY_MS,
    BRINGUP_MAX_DELAY_MS, EXIT_ABSENT, EXIT_GAVE_UP,
};

#[test]
fn every_attempt_before_the_last_sleeps_and_the_last_gives_up() {
    for done in 1..BRINGUP_ATTEMPTS {
        match next(done) {
            Next::Retry(ms) => assert!(ms > 0, "attempt {done} retried without sleeping"),
            Next::GiveUp => panic!("gave up after {done} of {BRINGUP_ATTEMPTS}"),
        }
    }
    assert_eq!(next(BRINGUP_ATTEMPTS), Next::GiveUp);
}

#[test]
fn past_the_last_attempt_the_answer_stays_give_up() {
    for done in BRINGUP_ATTEMPTS..BRINGUP_ATTEMPTS + 64 {
        assert_eq!(next(done), Next::GiveUp);
    }
    assert_eq!(next(u32::MAX), Next::GiveUp);
}

#[test]
fn the_sleep_doubles_from_the_first_delay_and_holds_at_the_cap() {
    assert_eq!(delay_after(1), BRINGUP_FIRST_DELAY_MS);
    let mut prev = 0u64;
    for done in 1..200u32 {
        let d = delay_after(done);
        assert!(d >= prev, "delay shrank at {done}");
        assert!(d <= BRINGUP_MAX_DELAY_MS, "delay {d} past the cap at {done}");
        if prev != 0 && prev < BRINGUP_MAX_DELAY_MS {
            assert!(d == prev * 2 || d == BRINGUP_MAX_DELAY_MS);
        }
        prev = d;
    }
    assert_eq!(delay_after(u32::MAX), BRINGUP_MAX_DELAY_MS);
}

#[test]
fn zero_failures_wait_for_nothing() {
    assert_eq!(delay_after(0), 0);
}

#[test]
fn the_whole_schedule_is_bounded_and_short() {
    let total = total_sleep_ms();
    let summed: u64 = (1..BRINGUP_ATTEMPTS).map(delay_after).sum();
    assert_eq!(total, summed);
    // Long enough for a device that settles slowly, short enough that a
    // device that never comes up is let go within the old ten-second window.
    assert!(total >= 1_000, "schedule {total} ms gives a slow device no room");
    assert!(total <= 10_000, "schedule {total} ms holds a dead device too long");
}

#[test]
fn absent_and_gave_up_are_distinct_nonzero_codes() {
    assert_ne!(EXIT_ABSENT, 0);
    assert_ne!(EXIT_GAVE_UP, 0);
    assert_ne!(EXIT_ABSENT, EXIT_GAVE_UP);
}
