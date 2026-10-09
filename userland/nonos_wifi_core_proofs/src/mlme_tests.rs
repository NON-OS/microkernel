// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! The MLME against a simulated access point: a WPA2 join and a WPA3-SAE join
//! (hash-to-element and hunting and pecking) run to Connected with the keys
//! the AP derives; the client picks SAE on a transition network; a network
//! saved as WPA3 is refused when it shows PSK only; frames from another BSS
//! or for another station are ignored; refusals, a deauthentication, an
//! open network, a TKIP group cipher and an HT-only AP each end the join with
//! their own reason; and the association request offers the AP's own rates.

use crate::ap_sim::{gtk_kde, message1, message3, AP_RSNE_MIXED, GTK};
use nonos_wifi_core::dot11::auth::parse_auth;
use nonos_wifi_core::dot11::header::{frame_control, write_header, MAC_HEADER_LEN, TYPE_MGMT};
use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::mlme::{Entropy, JoinRequest, Mlme, MlmeFailure, MlmeState, SAE_ENTROPY};
use nonos_wifi_core::rsn::{JoinPolicy, SelectError};
use nonos_wifi_core::sae::h2e::{derive_pt, pwe_from_pt};
use nonos_wifi_core::sae::hnp::derive_pwe;
use nonos_wifi_core::sae::{SaeStation, SaeStep};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::RSN_IE;

const STA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
const AP: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
const OTHER: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x99];
const SSID: &[u8] = b"home";
const PASS: &[u8] = b"correct horse";
const SNONCE: [u8; 32] = [0x52; 32];

fn entropy() -> Entropy {
    let mut sae = [0u8; SAE_ENTROPY];
    for (i, b) in sae.iter_mut().enumerate() {
        *b = (i as u8).wrapping_mul(37).wrapping_add(11);
    }
    Entropy { snonce: SNONCE, sae }
}

fn join(policy: JoinPolicy) -> Mlme {
    Mlme::join(&JoinRequest { our_mac: STA, ssid: SSID, passphrase: PASS, policy, entropy: entropy() })
}

// A beacon from `bssid` carrying the given elements after SSID and DS params.
fn beacon(bssid: [u8; 6], elements: &[&[u8]]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, 8), [0xFF; 6], bssid, bssid, 0).unwrap();
    f.extend_from_slice(&[0u8; 8]);
    f.extend_from_slice(&[0x64, 0, 0x11, 0x04]);
    f.extend_from_slice(&[0, SSID.len() as u8]);
    f.extend_from_slice(SSID);
    f.extend_from_slice(&[3, 1, 6]);
    for e in elements {
        f.extend_from_slice(e);
    }
    f
}

fn rsne(akms: &[u8], caps: u16, group: u8) -> Vec<u8> {
    let mut b = vec![1, 0, 0x00, 0x0f, 0xac, group, 1, 0, 0x00, 0x0f, 0xac, 4];
    b.extend_from_slice(&(akms.len() as u16).to_le_bytes());
    for a in akms {
        b.extend_from_slice(&[0x00, 0x0f, 0xac, *a]);
    }
    b.extend_from_slice(&caps.to_le_bytes());
    let mut e = vec![48, b.len() as u8];
    e.extend_from_slice(&b);
    e
}

// A management frame from `src` (also the BSSID) to `dst` with `body`.
fn mgmt(subtype: u8, src: [u8; 6], dst: [u8; 6], body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; MAC_HEADER_LEN];
    write_header(&mut f, frame_control(TYPE_MGMT, subtype), dst, src, src, 0).unwrap();
    f.extend_from_slice(body);
    f
}

fn open_auth_ok(src: [u8; 6], dst: [u8; 6]) -> Vec<u8> {
    mgmt(11, src, dst, &[0, 0, 2, 0, 0, 0])
}

// An SAE Authentication frame from the AP to the station.
fn sae_from_ap(seq: u16, status: u16, body: &[u8]) -> Vec<u8> {
    let mut b = vec![3, 0];
    b.extend_from_slice(&seq.to_le_bytes());
    b.extend_from_slice(&status.to_le_bytes());
    b.extend_from_slice(body);
    mgmt(11, AP, STA, &b)
}

fn assoc_resp(status: u16) -> Vec<u8> {
    let mut body = vec![0x11, 0x04];
    body.extend_from_slice(&status.to_le_bytes());
    body.extend_from_slice(&0xC001u16.to_le_bytes());
    mgmt(1, AP, STA, &body)
}

// The elements of an association request, from after its fixed fields.
fn assoc_elements(req: &[u8]) -> Vec<(u8, Vec<u8>)> {
    let mut out = Vec::new();
    let mut off = MAC_HEADER_LEN + 4;
    while off + 2 <= req.len() {
        let len = req[off + 1] as usize;
        out.push((req[off], req[off + 2..off + 2 + len].to_vec()));
        off += 2 + len;
    }
    out
}

#[test]
fn a_wpa2_join_runs_to_connected() {
    let psk_rsne = rsne(&[2], 0, 4);
    let mut m = join(JoinPolicy::ANY);
    let auth = m.on_mgmt(&beacon(AP, &[&psk_rsne])).tx.expect("open authentication");
    let a = parse_auth(&auth, &AP, &STA);
    assert!(a.is_none(), "the request is to the AP, not from it");
    assert_eq!(&auth[4..10], &AP, "addressed to the AP");
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.expect("association request");
    let els = assoc_elements(&req);
    assert_eq!(els[0], (0, SSID.to_vec()));
    let rsn: Vec<u8> = els.iter().find(|e| e.0 == 48).map(|e| e.1.clone()).unwrap();
    assert_eq!(rsn, RSN_IE[2..].to_vec(), "the WPA2-PSK RSNE");
    assert!(m.on_mgmt(&assoc_resp(0)).tx.is_none());
    assert_eq!(m.state(), MlmeState::FourWay);

    let key = pmk(PASS, SSID);
    let anonce = [0xA1; 32];
    let m2 = m.on_eapol(&message1(Akm::Psk, 1, &anonce)).tx.expect("message 2");
    let ptk = Akm::Psk.derive_ptk(&key, &AP, &STA, &anonce, &SNONCE);
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2));
    let mut kd = psk_rsne.clone();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(m.on_eapol(&message3(Akm::Psk, 2, &anonce, &ptk, &kd)).tx.is_some());
    assert_eq!(m.state(), MlmeState::Connected);
    assert_eq!(m.tk(), Some(&ptk[32..48]));
    assert_eq!(m.akm(), Some(Akm::Psk));
    assert!(m.supplicant().is_some(), "the data path takes the supplicant over");
}

// Run a WPA3 join against an AP built from the same SAE code.
fn sae_join(h2e: bool) -> (Mlme, [u8; 32]) {
    let mut els: Vec<Vec<u8>> = vec![AP_RSNE_MIXED.to_vec()];
    if h2e {
        els.push(vec![244, 1, 0x20]);
    }
    let refs: Vec<&[u8]> = els.iter().map(|e| e.as_slice()).collect();
    let mut m = join(JoinPolicy::ANY);
    let commit = m.on_mgmt(&beacon(AP, &refs)).tx.expect("an SAE commit");
    let pwe = if h2e {
        pwe_from_pt(&derive_pt(SSID, PASS, None).unwrap(), &AP, &STA).unwrap()
    } else {
        derive_pwe(PASS, &vec![9u8; PASS.len()], &AP, &STA).unwrap()
    };
    let mut ap = SaeStation::start(pwe, h2e, &[0x33; 32], &[0x44; 32]).unwrap();
    // The station's commit, as the AP reads it.
    let sta = parse_auth(&commit, &AP, &STA).is_none();
    assert!(sta);
    let body = &commit[MAC_HEADER_LEN..];
    assert_eq!(u16::from_le_bytes([body[0], body[1]]), 3, "algorithm SAE");
    let status = u16::from_le_bytes([body[4], body[5]]);
    assert_eq!(status, if h2e { 126 } else { 0 });
    let SaeStep::Send { body: ap_confirm, .. } = ap.on_frame(1, status, &body[6..]) else {
        panic!("the AP accepts the station's commit");
    };
    let (_, ap_status, ap_commit) = ap.commit_frame();
    let confirm = m.on_mgmt(&sae_from_ap(1, ap_status, &ap_commit)).tx.expect("the confirm");
    let cb = &confirm[MAC_HEADER_LEN..];
    assert!(matches!(ap.on_frame(2, 0, &cb[6..]), SaeStep::Accepted), "the AP verifies it");
    let req = m.on_mgmt(&sae_from_ap(2, 0, &ap_confirm)).tx.expect("association");
    let els = assoc_elements(&req);
    let rsn = els.iter().find(|e| e.0 == 48).unwrap().1.clone();
    assert_eq!(rsn[17], 8, "AKM SAE");
    assert_eq!(rsn[18] & 0x80, 0x80, "MFPC");
    assert_eq!(els.iter().any(|e| e.0 == 244), h2e, "the RSNXE rides with H2E");
    m.on_mgmt(&assoc_resp(0));
    (m, ap.pmk().unwrap().0)
}

#[test]
fn a_wpa3_join_runs_sae_then_the_four_way_to_connected() {
    for h2e in [true, false] {
        let (mut m, pmk) = sae_join(h2e);
        assert_eq!(m.state(), MlmeState::FourWay);
        let anonce = [0xA2; 32];
        let m2 = m.on_eapol(&message1(Akm::Sae, 1, &anonce)).tx.expect("message 2");
        let ptk = Akm::Sae.derive_ptk(&pmk, &AP, &STA, &anonce, &SNONCE);
        assert!(verify_mic_kind(Akm::Sae.mic_kind(), &ptk[..16], &m2), "the SAE PMK keys it");
        let mut kd = AP_RSNE_MIXED.to_vec();
        if h2e {
            kd.extend_from_slice(&[244, 1, 0x20]);
        }
        kd.extend_from_slice(&gtk_kde(1, &GTK));
        assert!(m.on_eapol(&message3(Akm::Sae, 2, &anonce, &ptk, &kd)).tx.is_some());
        assert_eq!(m.state(), MlmeState::Connected, "h2e={h2e}");
        assert_eq!(m.akm(), Some(Akm::Sae));
    }
}

#[test]
fn a_network_saved_as_wpa3_is_refused_when_it_shows_psk_only() {
    let mut m = join(JoinPolicy::WPA3_ONLY);
    assert!(m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)])).tx.is_none(), "nothing is sent");
    assert_eq!(m.failure(), Some(MlmeFailure::Select(SelectError::Downgrade)));
}

#[test]
fn frames_from_another_bss_or_for_another_station_are_ignored() {
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    // A refusal from another AP, and the right AP refusing another station.
    let refuse_other = mgmt(11, OTHER, STA, &[0, 0, 2, 0, 1, 0]);
    let refuse_them = mgmt(11, AP, OTHER, &[0, 0, 2, 0, 1, 0]);
    assert!(m.on_mgmt(&refuse_other).tx.is_none());
    assert!(m.on_mgmt(&refuse_them).tx.is_none());
    assert_eq!(m.state(), MlmeState::Authenticating, "neither ended the join");
    assert!(m.on_mgmt(&open_auth_ok(AP, STA)).tx.is_some());
    let mut other_resp = assoc_resp(17);
    other_resp[10..16].copy_from_slice(&OTHER);
    other_resp[16..22].copy_from_slice(&OTHER);
    m.on_mgmt(&other_resp);
    assert_eq!(m.state(), MlmeState::Associating);
}

#[test]
fn refusals_end_the_join_with_their_codes() {
    let psk_rsne = rsne(&[2], 0, 4);
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&mgmt(11, AP, STA, &[0, 0, 2, 0, 17, 0]));
    assert_eq!(m.failure(), Some(MlmeFailure::AuthRejected(17)));

    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(30));
    assert_eq!(m.state(), MlmeState::Associating, "status 30 waits for the retransmit");
    m.on_mgmt(&assoc_resp(18));
    assert_eq!(m.failure(), Some(MlmeFailure::AssocRejected(18)));

    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&psk_rsne]));
    m.on_mgmt(&open_auth_ok(AP, STA));
    m.on_mgmt(&assoc_resp(0));
    m.on_mgmt(&mgmt(12, AP, STA, &[15, 0]));
    assert_eq!(m.failure(), Some(MlmeFailure::Left(15)), "deauthenticated in the four-way");
}

#[test]
fn unjoinable_networks_say_why() {
    let cases: [(Vec<Vec<u8>>, MlmeFailure); 4] = [
        (vec![], MlmeFailure::OpenNetwork),
        (vec![rsne(&[2], 0, 2)], MlmeFailure::Select(SelectError::UnsupportedCipher)),
        (vec![vec![48, 3, 1, 0, 0]], MlmeFailure::MalformedRsne),
        (vec![rsne(&[2], 0, 4), vec![1, 2, 0x82, 0xFF]], MlmeFailure::NeedsHt),
    ];
    for (els, want) in cases {
        let refs: Vec<&[u8]> = els.iter().map(|e| e.as_slice()).collect();
        let mut m = join(JoinPolicy::ANY);
        assert!(m.on_mgmt(&beacon(AP, &refs)).tx.is_none());
        assert_eq!(m.failure(), Some(want));
    }
    let mut m = Mlme::join(&JoinRequest {
        our_mac: STA,
        ssid: SSID,
        passphrase: b"short",
        policy: JoinPolicy::ANY,
        entropy: entropy(),
    });
    m.on_mgmt(&beacon(AP, &[&rsne(&[2], 0, 4)]));
    assert_eq!(m.failure(), Some(MlmeFailure::BadPassphrase));
}

#[test]
fn the_association_request_offers_the_aps_own_rates() {
    // A g-only AP: OFDM rates only, 6, 12 and 24 basic.
    let sr = [1u8, 8, 0x8c, 0x12, 0x98, 0x24, 0xb0, 0x48, 0x60, 0x6c];
    let mut m = join(JoinPolicy::ANY);
    m.on_mgmt(&beacon(AP, &[&sr, &rsne(&[2], 0, 4)]));
    let req = m.on_mgmt(&open_auth_ok(AP, STA)).tx.unwrap();
    let els = assoc_elements(&req);
    assert_eq!(els[1], (1, sr[2..].to_vec()), "its eight rates, basic bits and all");
    assert!(!els.iter().any(|e| e.0 == 50), "no extended rates when eight suffice");
}
