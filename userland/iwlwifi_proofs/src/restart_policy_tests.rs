// NONOS Operating System (AGPL-3.0-or-later)
//! W4: the firmware restarts are bounded (three), rested (1 s, 10 s, 60 s)
//! and forgiven once the radio ran ten minutes without a failure.

use crate::restarts::{Restarts, FORGIVE_MS};

#[test]
fn restarts_are_bounded_rested_and_forgiven() {
    let mut r = Restarts::NEW;
    assert_eq!(r.after_failure(5_000), Some(6_000));
    assert_eq!(r.after_failure(7_000), Some(17_000));
    assert_eq!(r.after_failure(20_000), Some(80_000));
    assert_eq!((r.after_failure(90_000), r.tries()), (None, 3), "three tries, then down");
    assert_eq!(r.after_failure(20_000 + FORGIVE_MS - 1), None, "not forgiven a moment early");
    assert_eq!(
        r.after_failure(20_000 + FORGIVE_MS),
        Some(21_000 + FORGIVE_MS),
        "ten quiet minutes"
    );
    assert_eq!(r.tries(), 1);
    let mut late = Restarts::NEW;
    assert_eq!(late.after_failure(u64::MAX - 10), Some(u64::MAX), "the rest saturates");
}
