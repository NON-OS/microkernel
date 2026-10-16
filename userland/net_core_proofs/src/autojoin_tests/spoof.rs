// NONOS Operating System (AGPL-3.0-or-later)
//! A refusal anyone on the air can forge costs time, never the network: a
//! join ended by a deauthentication (code -6) is tried again later, in a
//! rejoin and at boot, within a bound; one refused for good is not.

use super::rejoin::lost_at;
use crate::autojoin_machine::{
    step_for, Due, Machine, Step, REJOIN_ATTEMPTS, REJOIN_FIRST_MS, REJOIN_MAX_MS,
};

const DEAUTH: i32 = -6;

#[test]
fn a_rejoin_refused_twice_by_deauth_is_tried_again_and_joins() {
    let mut m = lost_at(0);
    let mut t = 0;
    for _ in 0..2 {
        t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
        m.after_attempt(t, Step::Started(1));
        assert_eq!(m.join_ended(t + 1, DEAUTH), Some((1, false)));
        assert!(m.rejoining() && !m.done(), "still trying after a deauth");
        assert!(m.due(t + 1) == Due::Nothing, "but not at once");
    }
    t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    m.after_attempt(t, Step::Started(1));
    assert_eq!(m.join_ended(t + 1, 0), Some((1, true)));
}

#[test]
fn at_boot_a_network_refused_by_deauth_comes_round_again_with_a_growing_wait() {
    let mut m = Machine::new();
    let mut t = 0;
    let mut waits = Vec::new();
    for _ in 0..REJOIN_ATTEMPTS {
        m.after_attempt(t, step_for(0, DEAUTH));
        assert_eq!(m.tried(), 1, "tried this round");
        t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
        m.after_attempt(t, Step::Done);
        assert!(!m.done() && m.tried() == 0, "a new round");
        let next = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
        waits.push(next - t);
        t = next;
    }
    let want: Vec<i64> =
        (1..=REJOIN_ATTEMPTS as u32).map(|r| (REJOIN_FIRST_MS << r).min(REJOIN_MAX_MS)).collect();
    assert_eq!(waits, want);
    m.after_attempt(t, step_for(0, DEAUTH));
    let t = (t..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    m.after_attempt(t, Step::Done);
    assert!(m.done(), "the rounds are bounded");
}

#[test]
fn at_boot_a_network_refused_for_good_does_not_come_round() {
    let mut m = Machine::new();
    m.after_attempt(0, step_for(0, -9));
    let t = (0..).find(|&t| m.due(t) == Due::Attempt).unwrap();
    m.after_attempt(t, Step::Done);
    assert!(m.done(), "a passphrase of the wrong length is not tried again");
}
