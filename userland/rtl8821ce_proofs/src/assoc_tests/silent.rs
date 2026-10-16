// NONOS Operating System (AGPL-3.0-or-later)

//! A join against a silent radio times out rather than hanging.

use super::beacons::{beacon_for, OUR};
use crate::assoc::{run, Outcome, Radio};

// A radio that never delivers a frame, to prove the join gives up instead of
// spinning forever.
struct SilentRadio {
    sent: usize,
}

impl Radio for SilentRadio {
    fn send(&mut self, _mpdu: &[u8]) -> bool {
        self.sent += 1;
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

#[test]
fn a_join_against_a_silent_radio_times_out() {
    let mut radio = SilentRadio { sent: 0 };
    // A minimal beacon for "net" with an RSN element, enough for the machine to
    // select the BSS and emit the auth request (which the radio swallows).
    let beacon = beacon_for(b"net");
    let report = run(&mut radio, OUR, b"net", b"password123", &beacon, [0x11u8; 32], 32);
    assert!(matches!(report.outcome, Outcome::TimedOut), "a silent AP ends in a timeout");
    assert_eq!(report.sent, 1, "the auth request was transmitted from the beacon");
    assert_eq!(report.recv, 0, "a silent radio delivered nothing");
    assert_eq!(report.state, 1, "the machine reached Authenticating and waited");
    assert!(radio.sent >= 1, "the radio saw the transmit");
}
