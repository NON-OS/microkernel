// NONOS Operating System (AGPL-3.0-or-later)
//! Proofs for the association radio glue (`assoc`): a management frame is
//! classified as management, an EAPOL-carrying data frame is unwrapped to its
//! payload, anything else is ignored, an EAPOL payload wraps into an 802.11 data
//! frame addressed to the AP, and a join against a silent radio times out rather
//! than hanging.

use crate::assoc::{
    classify, eapol_from_bss, run, run_join, wrap_eapol, Outcome, Radio, RxKind, RETX_AFTER_MS,
    RETX_SAE_AFTER_MS,
};
use nonos_wifi_core::dot11::data::{build_data, parse_data};
use nonos_wifi_core::mlme::{Entropy, JoinRequest, MlmeFailure, SAE_ENTROPY};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};

const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const AP: [u8; 6] = [0x02, 0, 0, 0, 0, 0xAA];

#[test]
fn a_management_frame_is_classified_as_management() {
    // Frame control type bits = management (0). A beacon frame control is 0x0080.
    let frame = [0x80u8, 0x00, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut eth = Vec::new();
    assert_eq!(classify(&frame, &mut eth), RxKind::Mgmt);
}

#[test]
fn an_eapol_data_frame_is_unwrapped_to_its_payload() {
    // Build an ethernet frame carrying an EAPOL payload, wrap it as an 802.11 data
    // frame, and confirm classify pulls the payload back out.
    let eapol = [0x02u8, 0x03, 0x00, 0x5F, 0xAB, 0xCD]; // EAPOL version/type + body
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR); // dst
    eth.extend_from_slice(&AP); // src
    eth.extend_from_slice(&0x888Eu16.to_be_bytes());
    eth.extend_from_slice(&eapol);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();

    let mut out = Vec::new();
    match classify(&mpdu, &mut out) {
        RxKind::Eapol(start, end) => assert_eq!(&out[start..end], &eapol),
        other => panic!("expected EAPOL, got {other:?}"),
    }
}

#[test]
fn a_non_eapol_data_frame_is_ignored() {
    // A data frame carrying IPv4 (ethertype 0x0800) is not part of the join.
    let mut eth = Vec::new();
    eth.extend_from_slice(&OUR);
    eth.extend_from_slice(&AP);
    eth.extend_from_slice(&0x0800u16.to_be_bytes());
    eth.extend_from_slice(&[1, 2, 3, 4]);
    let mpdu = build_data(&eth, OUR, AP, 0).unwrap();
    let mut out = Vec::new();
    assert_eq!(classify(&mpdu, &mut out), RxKind::Other);
}

#[test]
fn wrap_eapol_builds_a_data_frame_to_the_ap() {
    let eapol = [0x02u8, 0x03, 0x00, 0x10, 0x99];
    let mpdu = wrap_eapol(&eapol, OUR, AP, 7).unwrap();
    // It round-trips: parse the data frame back to ethernet and recover the EAPOL.
    let eth = parse_data(&mpdu).unwrap();
    assert_eq!(&eth[0..6], &AP, "destination is the AP");
    assert_eq!(&eth[6..12], &OUR, "source is the station");
    assert_eq!(u16::from_be_bytes([eth[12], eth[13]]), 0x888E, "ethertype is EAPOL");
    assert_eq!(&eth[14..], &eapol, "payload preserved");
}

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

// Build a beacon frame for `ssid` advertising RSN, the shape parse_beacon accepts.
fn beacon_for(ssid: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&[0x80, 0x00]); // frame control: beacon
    f.extend_from_slice(&[0, 0]); // duration
    f.extend_from_slice(&[0xFF; 6]); // DA broadcast
    f.extend_from_slice(&AP); // SA
    f.extend_from_slice(&AP); // BSSID
    f.extend_from_slice(&[0, 0]); // seq
    f.extend_from_slice(&[0u8; 8]); // timestamp
    f.extend_from_slice(&[0x64, 0]); // beacon interval
    f.extend_from_slice(&[0x11, 0x04]); // capability: ESS + privacy
    // SSID element.
    f.push(0);
    f.push(ssid.len() as u8);
    f.extend_from_slice(ssid);
    // DS parameter set: channel 6.
    f.extend_from_slice(&[3, 1, 6]);
    // RSN element (id 48), a minimal WPA2-PSK/CCMP body.
    let rsn: [u8; 20] = [
        0x01, 0x00, // version
        0x00, 0x0F, 0xAC, 0x04, // group cipher CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x04, // pairwise CCMP
        0x01, 0x00, 0x00, 0x0F, 0xAC, 0x02, // akm PSK
        0x00, 0x00, // rsn caps
    ];
    f.push(48);
    f.push(rsn.len() as u8);
    f.extend_from_slice(&rsn);
    f
}

// A radio that keeps what it was asked to send and never answers.
struct RecordingRadio {
    sent: Vec<Vec<u8>>,
}
impl Radio for RecordingRadio {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        self.sent.push(mpdu.to_vec());
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

// A clock that moves `step` milliseconds each time it is read; the join reads it
// once to start and once per receive pass.
fn clock(step: u64) -> impl FnMut() -> u64 {
    let mut t = 0u64;
    move || {
        let now = t;
        t += step;
        now
    }
}

// One millisecond a pass, so a budget of 32 is 32 passes as it was.
fn passes() -> impl FnMut() -> u64 {
    clock(1)
}

fn join_request(policy: JoinPolicy) -> JoinRequest<'static> {
    JoinRequest {
        our_mac: OUR,
        ssid: b"net",
        passphrase: b"password123",
        policy,
        entropy: Entropy { snonce: [0x11; 32], sae: [0x22; SAE_ENTROPY] },
    }
}

// `beacon_for`'s beacon with the RSN element's AKM suite type replaced and
// management frame protection capable (WPA3 requires it).
fn beacon_with_akm(akm: u8) -> Vec<u8> {
    let mut b = beacon_for(b"net");
    let at = b.len() - 3; // the AKM suite type, before the two capability octets
    b[at] = akm;
    b[at + 1] = 0x80; // RSN capabilities: MFPC
    b
}

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

#[test]
fn only_the_access_points_own_eapol_to_this_station_drives_the_handshake() {
    let mut f = vec![0x08, 0x02, 0, 0];
    f.extend_from_slice(&OUR);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&AP);
    f.extend_from_slice(&[0, 0]);
    assert!(eapol_from_bss(&f, &OUR, &AP), "FromDS, from the BSSID, to us, in the clear");
    let mut to_ds = f.clone();
    to_ds[1] = 0x01;
    assert!(!eapol_from_bss(&to_ds, &OUR, &AP), "ToDS: another station's");
    let mut other = f.clone();
    other[4] ^= 1;
    assert!(!eapol_from_bss(&other, &OUR, &AP), "to another station");
    let mut bss = f.clone();
    bss[10] ^= 1;
    assert!(!eapol_from_bss(&bss, &OUR, &AP), "from another BSS");
    let mut prot = f.clone();
    prot[1] |= 0x40;
    assert!(!eapol_from_bss(&prot, &OUR, &AP), "protected: not the handshake in the clear");
    assert!(!eapol_from_bss(&f[..20], &OUR, &AP), "a runt");
}

// The sequence number in a frame's sequence control field.
fn seq_of(frame: &[u8]) -> u16 {
    u16::from_le_bytes([frame[22], frame[23]]) >> 4
}

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

// An access point that applies 802.11 duplicate detection (802.11-2020,
// 10.3.2.14.3): a frame with the Retry bit set whose sequence control matches
// the last one it took from this station is discarded. It takes the request but
// its answer never reaches the station, as when the answer is lost or is
// status 30 while it checks an earlier protected association.
struct DedupAp {
    last: Option<[u8; 2]>,
    taken: usize,
    discarded: usize,
}
impl Radio for DedupAp {
    fn send(&mut self, mpdu: &[u8]) -> bool {
        let sc = [mpdu[22], mpdu[23]];
        if mpdu[1] & 0x08 != 0 && self.last == Some(sc) {
            self.discarded += 1;
        } else {
            self.taken += 1;
            self.last = Some(sc);
        }
        true
    }
    fn recv(&mut self, _out: &mut [u8]) -> Option<usize> {
        None
    }
}

// Every resend reaches an access point that took the first copy: none is
// discarded as a duplicate, so an answer lost once is asked for again.
#[test]
fn every_resend_reaches_an_access_point_that_took_the_first() {
    let mut ap = DedupAp { last: None, taken: 0, discarded: 0 };
    let mut now = clock(1);
    let report = run_join(&mut ap, &join_request(JoinPolicy::ANY), &beacon_for(b"net"), 1000, &mut now);
    assert_eq!(report.sent, 5);
    assert_eq!(ap.discarded, 0, "no resend is a duplicate of the first");
    assert_eq!(ap.taken, 5, "the access point took the first send and all four resends");
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
