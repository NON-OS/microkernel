// NONOS Operating System (AGPL-3.0-or-later)
//! The wait itself, on the proofs' clock.

use crate::clock::{idles, now_ms};
use crate::wait::{delay_ms, until_ms};

#[test]
fn a_wait_that_never_holds_ends_on_its_deadline_idling_between_looks() {
    let (start, idled) = (now_ms(), idles());
    let mut asked = 0u64;
    assert!(!until_ms(25, || {
        asked += 1;
        false
    }));
    let took = now_ms() - start;
    assert!((25..=27).contains(&took), "{took} ms, not a count of reads");
    let slept = idles() - idled;
    assert!((25..=27).contains(&slept), "a millisecond's idle between looks: {slept}");
    assert_eq!(asked, slept + 2, "a look after each idle, the first, and one past the end");
}

#[test]
fn a_delay_idles_through_its_time() {
    let (start, idled) = (now_ms(), idles());
    delay_ms(20);
    let slept = idles() - idled;
    assert!((20..=22).contains(&slept), "idled, never spun: {slept}");
    assert!(now_ms() - start >= 20);
}

#[test]
fn a_completion_that_lands_at_the_deadline_is_seen() {
    let end = now_ms() + 10;
    assert!(until_ms(10, || now_ms() > end), "the last look after the deadline");
}

#[test]
fn a_completion_in_time_ends_the_wait_at_once() {
    let start = now_ms();
    assert!(until_ms(1000, || now_ms() >= start + 2));
    assert!(now_ms() - start < 5);
}

#[test]
fn a_delay_lasts_at_least_its_time_and_never_less_than_a_tick() {
    let start = now_ms();
    delay_ms(50);
    assert!(now_ms() - start >= 50);
    let start = now_ms();
    delay_ms(0);
    assert!(now_ms() - start >= 1);
}
