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

//! The supplicant against IEEE Std 802.11-2020, 12.7.6 and 12.7.7: a repeated
//! message 1 is answered, message 3 must hold the beacon's RSNE (a downgraded
//! one fails the handshake), a forged or replayed frame is dropped without
//! ending it, a repeated message 3 gets message 4 again without reinstalling a
//! key, the group key handshake delivers the next GTK, the SAE AKM runs with
//! the AES-CMAC MIC and the SHA-256 PTK, and key data larger than a bare GTK
//! (a mixed WPA/WPA2 AP's, or with the IGTK) unwraps.

use crate::ap_sim::*;
use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::eapol::parse::{parse, KEY_INFO_ACK, KEY_INFO_REQUEST, KEY_INFO_SECURE};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::ptk::pmk;
use nonos_wifi_core::wpa::supplicant::{Config, Failure, State, Supplicant};

const PMK_SAE: [u8; 32] = [0x77; 32];

fn psk() -> [u8; 32] {
    pmk(b"ThisIsAPassword", b"ThisIsASSID")
}

fn configured(akm: Akm, pmk: [u8; 32], own_rsne: &[u8], ap_rsne: &[u8]) -> Supplicant {
    Supplicant::configure(&Config {
        pmk,
        aa: AA,
        spa: SPA,
        snonce: SNONCE,
        akm,
        own_rsne,
        own_rsnxe: None,
        ap_rsne,
        ap_rsnxe: None,
        pmf: akm == Akm::Sae,
    })
}

fn ptk_for(akm: Akm, pmk: &[u8; 32], anonce: &[u8; 32]) -> [u8; 48] {
    akm.derive_ptk(pmk, &AA, &SPA, anonce, &SNONCE)
}

#[test]
fn a_mixed_mode_message3_unwraps_and_message4_verifies() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    let m2 = sup.step(&message1(Akm::Psk, 1, &ANONCE)).reply.expect("message 2");
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2), "message 2 MIC under the KCK");
    let k2 = parse(&m2).unwrap();
    assert_eq!(k2.key_data, &STA_RSNE_PSK, "message 2 repeats the association RSNE");
    assert_eq!(k2.nonce, SNONCE);

    // RSNE, WPA element and GTK KDE: 70 bytes, more than eight wrap blocks.
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&WPA_IE);
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(pad(kd.clone()).len() > 64);
    let m4 = sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd)).reply.expect("message 4");
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.gtk(), &GTK);
    assert_eq!(sup.tk(), &ptk[32..48]);
    let k4 = parse(&m4).unwrap();
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m4), "message 4 MIC");
    assert_ne!(k4.key_info & KEY_INFO_SECURE, 0, "message 4 is Secure");
    assert_eq!(k4.nonce, [0u8; 32], "message 4 carries a zero nonce");
    assert_eq!(k4.replay_counter, rc(2), "message 4 echoes message 3's counter");
}

#[test]
fn a_repeated_message1_is_answered_with_its_own_anonce() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    // Message 2 was lost; the AP repeats message 1 with a new counter and ANonce.
    let anonce2 = [0xB2; 32];
    let m2 = sup.step(&message1(Akm::Psk, 2, &anonce2)).reply.expect("a second message 2");
    assert_eq!(parse(&m2).unwrap().replay_counter, rc(2));
    let ptk = ptk_for(Akm::Psk, &psk(), &anonce2);
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2), "keyed by the new ANonce");
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(sup.step(&message3(Akm::Psk, 3, &anonce2, &ptk, &kd)).reply.is_some());
    assert_eq!(sup.state(), State::Connected);
}

#[test]
fn a_downgraded_rsne_in_message3_fails_the_handshake() {
    // The beacon offered PSK and SAE; message 3 signs an RSNE with PSK alone.
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = STA_RSNE_PSK.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    assert!(sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd)).reply.is_none());
    assert_eq!(sup.state(), State::Failed);
    assert_eq!(sup.failure(), Some(Failure::IeMismatch));
}

#[test]
fn a_forged_or_replayed_message3_is_dropped_without_ending_the_handshake() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 5, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    let mut forged = message3(Akm::Psk, 6, &ANONCE, &ptk, &kd);
    forged[90] ^= 1; // a bit of the MIC
    assert!(sup.step(&forged).reply.is_none());
    assert_eq!(sup.state(), State::PtkDerived, "a forged frame does not end the join");
    let mut wrong_anonce = message3(Akm::Psk, 6, &[0xEE; 32], &ptk, &kd);
    assert!(sup.step(&wrong_anonce).reply.is_none(), "another ANonce is not message 3");
    wrong_anonce.clear();
    let good = message3(Akm::Psk, 6, &ANONCE, &ptk, &kd);
    assert!(sup.step(&good).reply.is_some());
    // The same message 3 again (same counter) is a replay and is dropped.
    assert!(sup.step(&good).reply.is_none(), "replayed counter dropped");
    assert_eq!(sup.state(), State::Connected);
}

#[test]
fn a_repeated_message3_gets_message4_again_without_reinstalling() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));
    // The AP did not get message 4 and repeats message 3 with the next counter.
    let mut kd2 = AP_RSNE_MIXED.to_vec();
    kd2.extend_from_slice(&gtk_kde(2, &[0x99; 16]));
    let out = sup.step(&message3(Akm::Psk, 3, &ANONCE, &ptk, &kd2));
    let m4 = out.reply.expect("message 4 again");
    assert_eq!(parse(&m4).unwrap().replay_counter, rc(3));
    assert!(!out.new_group_key, "nothing is reinstalled");
    assert_eq!(sup.gtk(), &GTK, "the group key in use is kept");
    assert_eq!(sup.gtk_id(), 1);
}

#[test]
fn the_group_key_handshake_delivers_the_next_gtk() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    sup.step(&message1(Akm::Psk, 1, &ANONCE));
    let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &kd));

    let next = [0x22u8; 16];
    let g1 = group1(Akm::Psk, 3, &ptk, &gtk_kde(2, &next));
    let out = sup.step(&g1);
    let g2 = out.reply.expect("group message 2");
    assert!(out.new_group_key, "a new key to install");
    assert_eq!((sup.gtk(), sup.gtk_id()), (&next[..], 2));
    let k = parse(&g2).unwrap();
    assert!(verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &g2));
    assert_eq!(k.replay_counter, rc(3));
    assert_eq!(k.key_data.len(), 0, "group message 2 carries no key data");
    // A replay of the same group message is dropped.
    assert!(sup.step(&g1).reply.is_none());
    // The same key resent with a fresh counter is answered, not reinstalled.
    let again = sup.step(&group1(Akm::Psk, 4, &ptk, &gtk_kde(2, &next)));
    assert!(again.reply.is_some() && !again.new_group_key);
}

#[test]
fn the_sae_akm_runs_with_aes_cmac_and_the_sha256_ptk() {
    let sta_rsne: [u8; 22] = [
        0x30, 0x14, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01,
        0x00, 0x00, 0x0f, 0xac, 0x08, 0xc0, 0x00,
    ];
    let mut sup = configured(Akm::Sae, PMK_SAE, &sta_rsne, &AP_RSNE_MIXED);
    let m2 = sup.step(&message1(Akm::Sae, 1, &ANONCE)).reply.expect("message 2");
    let ptk = ptk_for(Akm::Sae, &PMK_SAE, &ANONCE);
    assert_ne!(ptk, nonos_wifi_core::wpa::ptk::ptk(&PMK_SAE, &AA, &SPA, &ANONCE, &SNONCE));
    assert!(verify_mic_kind(Akm::Sae.mic_kind(), &ptk[..16], &m2), "AES-CMAC MIC");
    assert!(!verify_mic_kind(Akm::Psk.mic_kind(), &ptk[..16], &m2), "not HMAC-SHA1");
    assert_eq!(parse(&m2).unwrap().key_info & 0x7, 0, "key descriptor version 0");
    let mut kd = AP_RSNE_MIXED.to_vec();
    kd.extend_from_slice(&gtk_kde(1, &GTK));
    kd.extend_from_slice(&igtk_kde(&[0x44; 16]));
    assert!(sup.step(&message3(Akm::Sae, 2, &ANONCE, &ptk, &kd)).reply.is_some());
    assert_eq!(sup.state(), State::Connected);
    let (igtk, id, ipn) = sup.igtk().expect("the IGTK under PMF");
    assert_eq!((igtk, id, ipn), (&[0x44u8; 16], 4, [1, 0, 0, 0, 0, 0]));
    // A frame with the WPA2 descriptor version is not this handshake's.
    let mut sup2 = configured(Akm::Sae, PMK_SAE, &sta_rsne, &AP_RSNE_MIXED);
    assert!(sup2.step(&message1(Akm::Psk, 1, &ANONCE)).reply.is_none());
}

#[test]
fn frames_an_access_point_never_sends_are_dropped() {
    let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
    let mut no_ack = message1(Akm::Psk, 1, &ANONCE);
    let info = u16::from_be_bytes([no_ack[5], no_ack[6]]) & !KEY_INFO_ACK;
    no_ack[5..7].copy_from_slice(&info.to_be_bytes());
    assert!(sup.step(&no_ack).reply.is_none(), "no Key Ack");
    let mut request = message1(Akm::Psk, 1, &ANONCE);
    let info = u16::from_be_bytes([request[5], request[6]]) | KEY_INFO_REQUEST;
    request[5..7].copy_from_slice(&info.to_be_bytes());
    assert!(sup.step(&request).reply.is_none(), "Request bit");
    let mut descriptor = message1(Akm::Psk, 1, &ANONCE);
    descriptor[4] = 254;
    assert!(sup.step(&descriptor).reply.is_none(), "the WPA1 descriptor type");
    assert!(sup.step(&message1(Akm::Psk, 1, &ANONCE)[..60]).reply.is_none(), "truncated");
    assert_eq!(sup.state(), State::Start);
}

#[test]
fn authentic_key_data_without_a_valid_gtk_fails_the_handshake() {
    for bad_kd in [
        AP_RSNE_MIXED.to_vec(), // no GTK at all
        {
            let mut v = AP_RSNE_MIXED.to_vec();
            v.extend_from_slice(&[0xDD, 30, 0x00, 0x0f, 0xac, 0x01, 1, 0]); // runs past the end
            v
        },
    ] {
        let mut sup = configured(Akm::Psk, psk(), &STA_RSNE_PSK, &AP_RSNE_MIXED);
        sup.step(&message1(Akm::Psk, 1, &ANONCE));
        let ptk = ptk_for(Akm::Psk, &psk(), &ANONCE);
        assert!(sup.step(&message3(Akm::Psk, 2, &ANONCE, &ptk, &bad_kd)).reply.is_none());
        assert_eq!(sup.failure(), Some(Failure::BadKeyData));
    }
}
