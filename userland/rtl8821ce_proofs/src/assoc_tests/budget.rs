// NONOS Operating System (AGPL-3.0-or-later)

//! The join's milliseconds, and the interval between resends.

use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::beacon_for;
use super::radio::{clock, join_request, RecordingRadio};
use crate::assoc::resend::RETX_AFTER_MS;
use crate::assoc::{run_join, Outcome};

// The join gives up when its budget of milliseconds has passed, not after a
// count of passes: with a second between passes a 10 s budget ends after ten.
#[test]
fn the_join_ends_when_its_milliseconds_are_spent() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut now = clock(1000);
    let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), 10_000, &mut now);
    assert!(matches!(report.outcome, Outcome::TimedOut));
    // Every pass is a full retransmit interval late, so each of the nine passes
    // before the budget runs out resends once.
    assert_eq!(radio.sent.len(), 10);
}

// A frame answered before its retransmit interval is not resent: the quiet time
// restarts with every frame the station sends.
#[test]
fn no_resend_inside_the_interval() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut now = clock(1);
    let _ = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), RETX_AFTER_MS, &mut now);
    assert_eq!(radio.sent.len(), 1, "the budget ends at the interval, before any resend");
}
