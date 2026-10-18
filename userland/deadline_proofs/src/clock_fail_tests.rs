// NONOS Operating System (AGPL-3.0-or-later)
//! A failed clock read fails the wait: it expires a deadline being set and
//! one being checked, so a broken clock never turns a timeout into a hang.

use crate::time::{set_uptime, Deadline};

const EIO: i64 = -5;

#[test]
fn a_deadline_set_on_a_failed_read_is_already_expired() {
    let d = Deadline::from_read(None, 30_000);
    assert!(d.is_past(0));
    assert!(d.reached(Some(0)));
}

#[test]
fn a_failed_read_reaches_any_deadline() {
    assert!(Deadline::at(u64::MAX).reached(None));
    assert!(Deadline::from_read(Some(5), 1000).reached(None));
}

#[test]
fn a_good_read_counts_from_its_own_time() {
    let d = Deadline::from_read(Some(500), 1000);
    assert!(!d.reached(Some(1499)));
    assert!(d.reached(Some(1500)));
    assert!(Deadline::from_read(Some(u64::MAX - 1), 10).is_past(u64::MAX));
}

#[test]
fn the_live_path_expires_when_the_clock_fails_mid_wait() {
    set_uptime(1000);
    let d = Deadline::after_ms(250);
    assert!(!d.expired());
    set_uptime(EIO);
    assert!(d.expired());
    set_uptime(1250);
    assert!(d.expired());
}

#[test]
fn the_live_path_expires_when_the_clock_fails_at_the_start() {
    set_uptime(EIO);
    let d = Deadline::after_ms(60_000);
    set_uptime(0);
    assert!(d.expired());
}
