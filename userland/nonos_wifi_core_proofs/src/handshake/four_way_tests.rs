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

//! The supplicant against IEEE Std 802.11-2020, 12.7.6 and 12.7.7: key data
//! larger than a bare GTK (a mixed WPA/WPA2 AP's) unwraps and message 4
//! verifies, and a repeated message 1 is answered. The rest is in
//! `handshake::replay_tests`, `handshake::refusal_tests` and
//! `handshake::group_tests`.

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use nonos_wifi_core::eapol::mic::verify_mic_kind;
use nonos_wifi_core::eapol::parse::{parse, KEY_INFO_SECURE};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

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
