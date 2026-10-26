// NONOS Operating System (AGPL-3.0-or-later)
//! `latency`: the middle, the slow end and the worst of the last 64 keys.

use super::latency::{Latency, Spread, KEEP};

#[test]
fn nothing_timed_says_nothing() {
    assert_eq!(Latency::new().spread(), None);
}

#[test]
fn the_spread_is_read_from_the_sorted_times() {
    let mut l = Latency::new();
    for ms in [30, 10, 20, 50, 40] {
        l.push(ms);
    }
    assert_eq!(l.spread(), Some(Spread { p50: 30, p95: 40, max: 50, keys: 5 }));
}

#[test]
fn only_the_last_64_keys_count() {
    let mut l = Latency::new();
    l.push(1000);
    for _ in 0..KEEP {
        l.push(7);
    }
    assert_eq!(l.spread(), Some(Spread { p50: 7, p95: 7, max: 7, keys: KEEP }), "the slow one aged out");
}
