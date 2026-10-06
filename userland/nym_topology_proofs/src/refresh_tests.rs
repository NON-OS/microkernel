// NONOS Operating System (AGPL-3.0-or-later)
//! The node list is fetched again before a fetched one expires. On the old
//! tick the fetch stopped once a gateway and an exit were held, so
//! `a_held_list_is_fetched_again_before_it_expires` fails there and Nym
//! stopped sending an hour into the boot.

use crate::refresh::{fetch_due, REFRESH_BEFORE_MS};

const HOUR: u64 = 60 * 60 * 1000;
const FETCHED_AT: u64 = 1_790_000_000_000;
const UNTIL: u64 = FETCHED_AT + HOUR;

#[test]
fn a_list_missing_a_gateway_or_an_exit_is_fetched() {
    assert!(fetch_due(0, 0, None, None));
    assert!(fetch_due(0, 5, Some(UNTIL), Some(FETCHED_AT)));
    assert!(fetch_due(5, 0, Some(UNTIL), Some(FETCHED_AT)));
}

#[test]
fn a_fresh_list_is_left_alone() {
    assert!(!fetch_due(5, 5, Some(UNTIL), Some(FETCHED_AT)));
    assert!(!fetch_due(5, 5, Some(UNTIL), Some(UNTIL - REFRESH_BEFORE_MS - 1)));
}

#[test]
fn a_held_list_is_fetched_again_before_it_expires() {
    assert!(fetch_due(5, 5, Some(UNTIL), Some(UNTIL - REFRESH_BEFORE_MS)));
    assert!(fetch_due(5, 5, Some(UNTIL), Some(UNTIL - 1)), "the replacement lands while the old list routes");
    assert!(fetch_due(5, 5, Some(UNTIL), Some(UNTIL)));
    assert!(fetch_due(5, 5, Some(UNTIL), Some(UNTIL + 3 * HOUR)), "an expired list keeps being fetched");
}

#[test]
fn an_image_or_signed_list_is_not_replaced_on_a_clock() {
    assert!(!fetch_due(5, 5, None, Some(u64::MAX)));
}

#[test]
fn an_unreadable_clock_fetches_nothing_extra() {
    assert!(!fetch_due(5, 5, Some(UNTIL), None));
}

#[test]
fn the_margin_sits_inside_the_list_lifetime() {
    const { assert!(REFRESH_BEFORE_MS < HOUR) };
    assert!(fetch_due(5, 5, Some(REFRESH_BEFORE_MS / 2), Some(0)), "no underflow near the epoch");
}
