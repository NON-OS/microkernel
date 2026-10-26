// NONOS Operating System (AGPL-3.0-or-later)

//! The join's milliseconds, and the interval between resends.

use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::beacon_for;
use super::radio::{clock, join_request, RecordingRadio};
use crate::assoc::resend::RETX_AFTER_MS;
use crate::assoc::{run_join, Outcome};
use nonos_wifi_core::mlme::AUTH_MS;

// The join gives up when its budget of milliseconds has passed, not after a
// count of passes: with a resend interval between passes, a budget inside
// the step's own time ends the join first, after four passes.
#[test]
fn the_join_ends_when_its_milliseconds_are_spent() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut now = clock(RETX_AFTER_MS);
    let budget = 4 * RETX_AFTER_MS;
    assert!(budget < AUTH_MS, "inside the authentication step's own time");
    let report =
        run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), budget, &mut now);
    assert!(matches!(report.outcome, Outcome::TimedOut));
    assert_eq!(report.failure, None, "the budget, not the step, ended it");
    // Every pass is a full retransmit interval late, so each of the three
    // passes before the budget runs out resends once.
    assert_eq!(radio.sent.len(), 4);
}

// A frame answered before its retransmit interval is not resent: the quiet time
// restarts with every frame the station sends.
#[test]
fn no_resend_inside_the_interval() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut now = clock(1);
    let _ = run_join(
        &mut radio,
        &join_request(JoinPolicy::ANY),
        &beacon_for(b"net"),
        RETX_AFTER_MS,
        &mut now,
    );
    assert_eq!(radio.sent.len(), 1, "the budget ends at the interval, before any resend");
}
