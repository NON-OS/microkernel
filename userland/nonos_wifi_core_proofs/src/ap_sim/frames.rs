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

//! The EAPOL-Key frames of the four-way and group key handshakes.

use super::wrap::{aes_wrap, pad};
use nonos_wifi_core::eapol::build::{build_key_frame_kind, KeyFrame};
use nonos_wifi_core::eapol::parse::{
    KEY_INFO_ACK, KEY_INFO_ENCRYPTED, KEY_INFO_INSTALL, KEY_INFO_MIC, KEY_INFO_PAIRWISE,
    KEY_INFO_SECURE,
};
use nonos_wifi_core::wpa::akm::Akm;

pub fn rc(n: u8) -> [u8; 8] {
    [0, 0, 0, 0, 0, 0, 0, n]
}

/// One EAPOL-Key frame from the AP under `kck` (an all-zero KCK for message 1,
/// which carries no MIC).
pub fn frame(akm: Akm, info: u16, replay: u8, nonce: &[u8; 32], kd: &[u8], kck: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 99 + kd.len()];
    let f = KeyFrame {
        key_info: akm.key_version() | info,
        replay_counter: &rc(replay),
        nonce,
        key_data: kd,
    };
    let n = build_key_frame_kind(&mut out, 2, akm.mic_kind(), &f, kck).unwrap();
    out.truncate(n);
    out
}

pub fn message1(akm: Akm, replay: u8, anonce: &[u8; 32]) -> Vec<u8> {
    frame(akm, KEY_INFO_PAIRWISE | KEY_INFO_ACK, replay, anonce, &[], &[0u8; 16])
}

/// Message 3 carrying `plain_kd`, wrapped under the PTK's KEK.
pub fn message3(akm: Akm, replay: u8, anonce: &[u8; 32], ptk: &[u8], plain_kd: &[u8]) -> Vec<u8> {
    let kek: [u8; 16] = ptk[16..32].try_into().unwrap();
    let wrapped = aes_wrap(&kek, &pad(plain_kd.to_vec()));
    let info = KEY_INFO_PAIRWISE
        | KEY_INFO_ACK
        | KEY_INFO_MIC
        | KEY_INFO_INSTALL
        | KEY_INFO_SECURE
        | KEY_INFO_ENCRYPTED;
    frame(akm, info, replay, anonce, &wrapped, &ptk[..16])
}

/// Group message 1 carrying `plain_kd`.
pub fn group1(akm: Akm, replay: u8, ptk: &[u8], plain_kd: &[u8]) -> Vec<u8> {
    let kek: [u8; 16] = ptk[16..32].try_into().unwrap();
    let wrapped = aes_wrap(&kek, &pad(plain_kd.to_vec()));
    let info = KEY_INFO_ACK | KEY_INFO_MIC | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;
    frame(akm, info, replay, &[0u8; 32], &wrapped, &ptk[..16])
}
