// NONOS Operating System (AGPL-3.0-or-later)
//! The watch sleeps a quiet link once a period, and keeps awake one that is
//! busy, whose access point went quiet, or that may not sleep.

use crate::ps::{Seen, Watch, BUSY_FRAMES, PERIOD_MS};

fn seen(tx: u32, rx: u32, sends: u32) -> Seen {
    Seen { tx, rx, sends }
}

#[test]
fn a_quiet_link_may_sleep_once_a_period_and_not_between() {
    let mut w = Watch::default();
    assert_eq!(w.due(1_000, seen(5, 5, 0), true), None, "the first look starts a period");
    assert_eq!(w.due(1_000 + PERIOD_MS - 1, seen(5, 5, 0), true), None);
    let b = BUSY_FRAMES;
    assert_eq!(w.due(1_000 + PERIOD_MS, seen(5 + b, 5 + b, 0), true), Some(true), "at the bound");
    assert_eq!(w.due(1_000 + 2 * PERIOD_MS - 1, seen(9_000, 0, 0), true), None);
}

#[test]
fn busy_traffic_a_quiet_ap_or_a_station_that_may_not_sleep_keeps_awake() {
    let t0 = 1_000;
    for (s, may, why) in [
        (seen(BUSY_FRAMES + 1, 0, 0), true, "transmit over the bound"),
        (seen(0, BUSY_FRAMES + 1, 0), true, "receive over the bound"),
        (seen(0, 0, 1), true, "the station probed its access point"),
        (seen(0, 0, 0), false, "the station may not sleep"),
    ] {
        let mut w = Watch::default();
        w.due(t0, seen(0, 0, 0), true);
        assert_eq!(w.due(t0 + PERIOD_MS, s, may), Some(false), "{why}");
    }
}

#[test]
fn counts_that_wrap_are_counted_across_the_wrap() {
    let mut w = Watch::default();
    w.due(0, seen(u32::MAX - 1, 0, 0), true);
    assert_eq!(w.due(PERIOD_MS, seen(3, 0, 0), true), Some(true), "five frames");
}
