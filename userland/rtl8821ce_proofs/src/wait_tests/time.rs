// NONOS Operating System (AGPL-3.0-or-later)
//! The wait itself, on the proofs' clock.

use crate::clock::now_ms;
use crate::wait::{delay_ms, until_ms};

#[test]
fn a_wait_that_never_holds_ends_on_its_deadline() {
    let start = now_ms();
    let mut asked = 0u32;
    assert!(!until_ms(25, || {
        asked += 1;
        false
    }));
    let took = now_ms() - start;
    assert!((25..=27).contains(&took), "{took} ms, not a count of reads");
    assert!(asked > 1000, "asked on and on until then: {asked}");
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
