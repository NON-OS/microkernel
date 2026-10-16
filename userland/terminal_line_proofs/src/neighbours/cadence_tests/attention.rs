// NONOS Operating System (AGPL-3.0-or-later)
//! Work arriving makes a quiet loop attentive, and one packet does not buy it all.

use super::fresh;
use crate::cadence::{next_wait, note_idle, note_work};

#[test]
fn a_quiet_loop_waits_the_long_wait() {
    let b = fresh();
    assert_eq!(next_wait(&b), 50);
}

#[test]
fn work_arriving_makes_the_loop_attentive() {
    let b = fresh();
    note_work(&b);
    assert_eq!(next_wait(&b), 2);
}

/// The property that matters: after one packet, the loop must not spend the
/// whole linger window at the fastest rate. Sixty-four turns at two
/// milliseconds is five hundred syscalls a second for an exchange that is
/// already over.
#[test]
fn one_packet_does_not_buy_the_whole_window_at_full_rate() {
    let b = fresh();
    note_work(&b);
    let mut fast = 0;
    let mut total_ms = 0u64;
    for _ in 0..64 {
        let w = next_wait(&b);
        if w == 2 {
            fast += 1;
        }
        total_ms += w;
        note_idle(&b);
    }
    assert_eq!(fast, 8, "attentive turns");
    // Same number of wake-ups, spread over far more wall time, so the rate
    // falls rather than the responsiveness.
    assert!(total_ms > 600, "window collapsed to {total_ms}ms");
}
