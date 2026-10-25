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

//! What fails or is dropped: message 3 must hold the beacon's RSNE (a
//! downgraded one fails the handshake), frames an access point never sends
//! are dropped, and authentic key data without a valid GTK fails.

use crate::ap_sim::*;
use crate::handshake::sim::{configured, psk, ptk_for};
use nonos_wifi_core::eapol::parse::{KEY_INFO_ACK, KEY_INFO_REQUEST};
use nonos_wifi_core::wpa::akm::Akm;
use nonos_wifi_core::wpa::supplicant::{Failure, State};

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
