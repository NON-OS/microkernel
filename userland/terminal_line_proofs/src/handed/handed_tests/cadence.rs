// NONOS Operating System (AGPL-3.0-or-later)
//! How often a new window asks the shell for a handed command.

use crate::handed_cadence::{next_ask, EVERY_MS, STARTUP_MS};

#[test]
fn a_new_window_asks_every_tick_then_twice_a_second() {
    let start = 50_000;
    assert_eq!(next_ask(start, start), start);
    assert_eq!(next_ask(start + STARTUP_MS - 1, start), start + STARTUP_MS - 1);
    assert_eq!(next_ask(start + STARTUP_MS, start), start + STARTUP_MS + EVERY_MS);
    assert_eq!(EVERY_MS, 500);
    // A clock read before the start does not stretch the quick phase forever.
    assert_eq!(next_ask(start - 10, start), start - 10);
    assert_eq!(next_ask(i64::MAX, 0), i64::MAX);
}
