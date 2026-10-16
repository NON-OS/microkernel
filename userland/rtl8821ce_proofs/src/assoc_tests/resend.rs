// NONOS Operating System (AGPL-3.0-or-later)

//! A lost frame resent by the clock, an SAE commit after the router's work.

use nonos_wifi_core::rsn::JoinPolicy;

use super::beacons::{beacon_for, beacon_with_akm};
use super::radio::{clock, join_request, seq_of, RecordingRadio};
use crate::assoc::{run_join, Outcome, RETX_AFTER_MS, RETX_SAE_AFTER_MS};

// The unanswered authentication frame is resent every RETX_AFTER_MS of the
// clock, however fast the receive passes run: at 1 ms and at 50 ms a pass the
// same second of a silent AP sees the first send and four resends. Each resend
// is a new frame, as mac80211 sends one: the next sequence number and the
// Retry bit clear, the same body.
#[test]
fn a_lost_frame_is_resent_on_the_clock_not_by_counting_passes() {
    assert_eq!(RETX_AFTER_MS, 200);
    for step in [1u64, 50] {
        let mut radio = RecordingRadio { sent: Vec::new() };
        let mut now = clock(step);
        let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), 1000, &mut now);
        assert!(matches!(report.outcome, Outcome::TimedOut));
        assert_eq!(radio.sent.len(), 5, "first send and resends at 200, 400, 600, 800 ms (step {step})");
        let first = &radio.sent[0];
        for (i, frame) in radio.sent.iter().enumerate() {
            assert_eq!(frame[1] & 0x08, 0, "no send carries the Retry bit");
            assert_eq!(seq_of(frame), i as u16, "each send takes the next sequence number");
            assert_eq!(&frame[..22], &first[..22], "the same header up to the sequence control");
            assert_eq!(&frame[24..], &first[24..], "and the same body");
        }
    }
}

// An SAE commit waits RETX_SAE_AFTER_MS for the router's computation before it
// is resent, so a slow router is not driven past its commit limit: in 5 s a
// silent router sees the commit and two resends, at 2 s and 4 s.
#[test]
fn an_sae_commit_is_resent_after_the_routers_computation() {
    assert_eq!(RETX_SAE_AFTER_MS, 2_000);
    let mut radio = RecordingRadio { sent: Vec::new() };
    let mut now = clock(1);
    let report = run_join(&mut radio, &join_request(JoinPolicy::ANY), &beacon_with_akm(8), 5_000, &mut now);
    assert_eq!(report.akm.map(|a| a.suite_type()), Some(8), "SAE was chosen");
    assert_eq!(radio.sent.len(), 3, "the commit and resends at 2 s and 4 s");
    for (i, frame) in radio.sent.iter().enumerate() {
        assert_eq!(u16::from_le_bytes([frame[24], frame[25]]), 3, "an SAE authentication frame");
        assert_eq!(u16::from_le_bytes([frame[26], frame[27]]), 1, "the commit");
        assert_eq!(seq_of(frame), i as u16, "each a new frame");
    }
}
