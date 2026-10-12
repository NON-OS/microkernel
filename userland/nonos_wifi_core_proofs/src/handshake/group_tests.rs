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

//! The group key handshake delivers the next GTK, and the SAE AKM runs with
//! the AES-CMAC MIC and the SHA-256 PTK, its key data carrying the IGTK.

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for, PMK_SAE};
use nonos_wifi_core::eapol::{mic::verify_mic_kind, parse::parse};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::State;

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
