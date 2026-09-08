// NONOS Operating System (AGPL-3.0-or-later)
//! The net.core serve loop's wait cadence.
//!
//! This decides how often an idle networking stack wakes the scheduler, which
//! is the difference between a machine that looks idle and one that reports
//! most of a core in use.

use core::sync::atomic::{AtomicU32, Ordering};

use crate::cadence::{next_wait, note_idle, note_work};

fn fresh() -> AtomicU32 {
    AtomicU32::new(0)
}

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

/// The step down. Full attention covers a handshake's gaps; the long tail
/// after it does not need the same rate.
#[test]
fn attention_decays_through_a_middle_step_rather_than_a_cliff() {
    let b = fresh();
    note_work(&b);
    let mut seen = alloc::vec::Vec::new();
    for _ in 0..70 {
        seen.push(next_wait(&b));
        note_idle(&b);
    }
    assert_eq!(seen[0], 2, "first turn after work should be attentive");
    assert!(seen.contains(&12), "no settling step in the decay");
    assert_eq!(*seen.last().unwrap(), 50, "should reach the quiet wait");
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

#[test]
fn work_during_the_tail_restores_full_attention() {
    let b = fresh();
    note_work(&b);
    for _ in 0..40 {
        note_idle(&b);
    }
    assert_eq!(next_wait(&b), 12);
    note_work(&b);
    assert_eq!(next_wait(&b), 2);
}

#[test]
fn idling_past_the_end_stays_quiet_and_does_not_wrap() {
    let b = fresh();
    note_work(&b);
    for _ in 0..500 {
        note_idle(&b);
    }
    assert_eq!(b.load(Ordering::Relaxed), 0);
    assert_eq!(next_wait(&b), 50);
}
