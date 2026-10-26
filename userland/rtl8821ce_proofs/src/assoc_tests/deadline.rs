// NONOS Operating System (AGPL-3.0-or-later)

//! A step the access point never answers ends the rtl join at that step's
//! own deadline, well inside the 8 s budget, as a timeout naming the step.

use nonos_wifi_core::mlme::{MlmeFailure, MlmeState, AUTH_MS};
use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::beacon_for;
use super::radio::{join_request, RecordingRadio};
use crate::assoc::{run_join, Outcome};

#[test]
fn a_silent_access_point_ends_the_join_at_the_steps_deadline() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut t = 0u64;
    let mut clock = || (t += 50, t).1;
    let report = run_join(
        &mut radio,
        &join_request(JoinPolicy::ANY),
        &beacon_for(b"net"),
        8_000,
        &mut clock,
    );
    assert!(matches!(report.outcome, Outcome::TimedOut));
    assert_eq!(report.failure, Some(MlmeFailure::Timeout(MlmeState::Authenticating)));
    assert!((AUTH_MS..AUTH_MS + 200).contains(&t), "{t} ms, not the budget's");
    assert!(radio.sent.len() >= 3, "the request resent inside the step's time");
}
