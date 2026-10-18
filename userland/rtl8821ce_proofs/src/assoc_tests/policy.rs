// NONOS Operating System (AGPL-3.0-or-later)

//! Which authentication a network's policy and offer lead to.

use nonos_wifi_core::mlme::MlmeFailure;
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

use super::beacons::{beacon_for, beacon_with_akm};
use super::radio::{join_request, passes, RecordingRadio};
use crate::assoc::{run_join, Outcome};

#[test]
fn sae_without_management_frame_protection_is_refused() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut beacon = beacon_with_akm(8);
    let caps = beacon.len() - 2;
    beacon[caps] = 0; // MFPC clear: not a valid WPA3 network
    let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon, 32, &mut passes());
    assert!(matches!(report.outcome, Outcome::Refused));
    // SAE is not on offer without MFPC, and there is no PSK to fall back to.
    assert_eq!(report.failure, Some(MlmeFailure::Select(SelectError::UnsupportedAkm)));
    assert!(radio.sent.is_empty());
}

#[test]
fn a_network_saved_as_wpa3_is_never_joined_with_wpa2() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let report = run_join(&mut radio, &join_request(JoinPolicy::WPA3_ONLY), &beacon_for(b"net"), 32, &mut passes());
    assert!(matches!(report.outcome, Outcome::Refused), "refused, not tried");
    assert_eq!(report.failure, Some(MlmeFailure::Select(SelectError::Downgrade)));
    assert!(radio.sent.is_empty(), "not one frame went to the WPA2-only network");
}

#[test]
fn a_network_offering_sae_is_joined_with_sae() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_with_akm(8), 32, &mut passes());
    assert!(matches!(report.outcome, Outcome::TimedOut), "the silent AP times out");
    assert_eq!(report.akm.map(|a| a.suite_type()), Some(8), "SAE was chosen");
    let first = radio.sent.first().expect("an authentication frame went out");
    assert_eq!(first[0], 0xB0, "an Authentication frame");
    assert_eq!(u16::from_le_bytes([first[24], first[25]]), 3, "algorithm 3: SAE");
    assert_eq!(u16::from_le_bytes([first[26], first[27]]), 1, "the commit");
}

#[test]
fn a_wpa2_network_is_joined_with_open_authentication_then_the_handshake() {
    let mut radio = RecordingRadio { sent: Vec::new() };
    let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), 32, &mut passes());
    assert_eq!(report.akm.map(|a| a.suite_type()), Some(2), "WPA2-PSK was chosen");
    let first = radio.sent.first().expect("an authentication frame went out");
    assert_eq!(u16::from_le_bytes([first[24], first[25]]), 0, "algorithm 0: Open System");
}
