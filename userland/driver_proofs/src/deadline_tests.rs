// NONOS Operating System (AGPL-3.0-or-later)
//! bring-up runs by the clock, not by a count of attempts.

use crate::bringup_deadline::{retry_after, BRINGUP_BUDGET_MS};
use crate::bringup_schedule::delay_after;

/// A driver whose every attempt takes `attempt_ms` and fails: how many
/// attempts it makes and when it gives up, on a modelled clock.
fn run(attempt_ms: u64) -> (u32, u64) {
    let (mut now, mut done) = (0u64, 0u32);
    loop {
        now += attempt_ms;
        done += 1;
        match retry_after(now, delay_after(done)) {
            Some(ms) => {
                assert!(ms > 0, "a retry always sleeps first");
                now += ms;
            }
            None => return (done, now),
        }
    }
}

#[test]
fn the_sleep_is_the_schedules_until_the_budget_cuts_it() {
    assert_eq!(retry_after(0, 100), Some(100));
    assert_eq!(retry_after(9_000, 3_200), Some(1_000), "cut to what is left");
    assert_eq!(retry_after(BRINGUP_BUDGET_MS, 100), None);
    assert_eq!(retry_after(u64::MAX, 100), None);
}

#[test]
fn an_attempt_that_fails_at_once_keeps_trying_for_the_whole_budget() {
    let (attempts, gave_up_at) = run(0);
    assert_eq!(gave_up_at, BRINGUP_BUDGET_MS);
    assert!(attempts > 7, "more than the old seven: {attempts}");
}

#[test]
fn an_attempt_that_waits_seconds_gives_up_near_the_budget_not_minutes_later() {
    let (attempts, gave_up_at) = run(5_000);
    assert!(gave_up_at < BRINGUP_BUDGET_MS + 5_000, "{gave_up_at}");
    assert!(attempts < 7, "{attempts}");
}
