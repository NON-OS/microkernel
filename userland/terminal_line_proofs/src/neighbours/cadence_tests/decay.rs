// NONOS Operating System (AGPL-3.0-or-later)
//! Attention decays in steps, comes back with work, and idling past the end stays quiet.

use super::fresh;
use crate::cadence::{next_wait, note_idle, note_work};
use core::sync::atomic::Ordering;

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
