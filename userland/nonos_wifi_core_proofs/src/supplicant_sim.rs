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

//! The access point side of the group key proofs: messages 1 and 3 of the
//! four-way handshake with a GTK KDE naming a chosen key index, and the
//! handshake that drives the real `Supplicant` through them.

use crate::ap_sim::wrap::aes_wrap;
use crate::ap_sim::{AA, ANONCE, SNONCE, SPA};
use nonos_wifi_core::eapol::build::build_key_frame;
use nonos_wifi_core::eapol::parse::{
    KEY_INFO_ACK, KEY_INFO_ENCRYPTED, KEY_INFO_INSTALL, KEY_INFO_MIC, KEY_INFO_PAIRWISE,
    KEY_INFO_SECURE, KEY_INFO_VERSION2,
};
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::supplicant::{State, Supplicant};

const GTK: [u8; 16] = [
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
];

// GTK KDE: dd <len> 00 0f ac 01 <key id / tx> <reserved> <gtk>, wrapped.
fn wrapped_gtk_kde(kek: &[u8; 16], key_id_byte: u8) -> Vec<u8> {
    let mut kde = vec![0xDD, 22, 0x00, 0x0f, 0xac, 0x01, key_id_byte, 0x00];
    kde.extend_from_slice(&GTK);
    aes_wrap(kek, &kde)
}

// The Key Information every access point sets: Key Ack on messages 1 and 3,
// and Install, Secure and Encrypted Key Data on message 3 (IEEE Std
// 802.11-2020, 12.7.6.2 and 12.7.6.4). The supplicant drops a message without
// them: an authenticator that leaves them out is not running the handshake
// the standard describes.
fn message1() -> Vec<u8> {
    let mut out = [0u8; 160];
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_ACK;
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 1], &ANONCE, &[], &[0u8; 16])
        .unwrap();
    out[..n].to_vec()
}

fn message3(kck: &[u8], kek: &[u8; 16], key_id_byte: u8) -> Vec<u8> {
    let mut out = [0u8; 160];
    let info = KEY_INFO_VERSION2 | KEY_INFO_PAIRWISE | KEY_INFO_MIC | KEY_INFO_ACK;
    let info = info | KEY_INFO_INSTALL | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;
    let kd = wrapped_gtk_kde(kek, key_id_byte);
    let n = build_key_frame(&mut out, info, &[0, 0, 0, 0, 0, 0, 0, 2], &ANONCE, &kd, kck).unwrap();
    out[..n].to_vec()
}

pub fn handshake(key_id_byte: u8) -> Supplicant {
    let key = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    let p = ptk(&key, &AA, &SPA, &ANONCE, &SNONCE);
    let mut sup = Supplicant::new(key, AA, SPA, SNONCE);
    sup.step(&message1());
    assert_eq!(sup.state(), State::PtkDerived);
    sup.step(&message3(&p[0..16], p[16..32].try_into().unwrap(), key_id_byte));
    assert_eq!(sup.state(), State::Connected);
    assert_eq!(sup.gtk(), &GTK);
    sup
}
