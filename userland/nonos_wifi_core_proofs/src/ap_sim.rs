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

//! The authenticator's half of the four-way and group key handshakes, built
//! the way IEEE Std 802.11-2020, 12.7.6 and 12.7.7 lay the messages out, so
//! the supplicant is driven by frames an access point would send.

use nonos_wifi_core::ccmp::aes::Aes128;
use nonos_wifi_core::eapol::build::{build_key_frame_kind, KeyFrame};
use nonos_wifi_core::eapol::parse::{
    KEY_INFO_ACK, KEY_INFO_ENCRYPTED, KEY_INFO_INSTALL, KEY_INFO_MIC, KEY_INFO_PAIRWISE,
    KEY_INFO_SECURE,
};
use nonos_wifi_core::wpa::akm::Akm;

pub const AA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x01];
pub const SPA: [u8; 6] = [0x02, 0x00, 0x00, 0x00, 0x00, 0x02];
pub const ANONCE: [u8; 32] = [0xA1; 32];
pub const SNONCE: [u8; 32] = [0x52; 32];
pub const GTK: [u8; 16] = [0x11; 16];

/// The WPA2-PSK RSNE the station sends (it equals `wpa::RSN_IE`).
pub const STA_RSNE_PSK: [u8; 22] = nonos_wifi_core::wpa::RSN_IE;
/// An AP beacon RSNE for a WPA2/WPA3 transition network: AKMs PSK and SAE,
/// MFPC set.
pub const AP_RSNE_MIXED: [u8; 26] = [
    0x30, 0x18, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x01, 0x00, 0x00, 0x0f, 0xac, 0x04, 0x02, 0x00,
    0x00, 0x0f, 0xac, 0x02, 0x00, 0x0f, 0xac, 0x08, 0x80, 0x00,
];
/// The legacy WPA vendor element a mixed WPA/WPA2 AP also signs into message 3.
pub const WPA_IE: [u8; 24] = [
    0xdd, 0x16, 0x00, 0x50, 0xf2, 0x01, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02, 0x01, 0x00, 0x00, 0x50,
    0xf2, 0x02, 0x01, 0x00, 0x00, 0x50, 0xf2, 0x02,
];

/// RFC 3394 AES key wrap: the AP's half of the group-key delivery.
pub fn aes_wrap(kek: &[u8; 16], plain: &[u8]) -> Vec<u8> {
    assert!(plain.len().is_multiple_of(8) && !plain.is_empty());
    let n = plain.len() / 8;
    let aes = Aes128::new(kek);
    let mut a = [0xA6u8; 8];
    let mut r: Vec<[u8; 8]> = (0..n).map(|i| plain[i * 8..i * 8 + 8].try_into().unwrap()).collect();
    for j in 0..6u64 {
        for (i, ri) in r.iter_mut().enumerate() {
            let mut block = [0u8; 16];
            block[..8].copy_from_slice(&a);
            block[8..].copy_from_slice(&ri[..]);
            aes.encrypt_block(&mut block);
            let t = (n as u64) * j + (i as u64) + 1;
            a.copy_from_slice(&block[..8]);
            for (k, ak) in a.iter_mut().enumerate() {
                *ak ^= (t >> (56 - 8 * k)) as u8;
            }
            ri.copy_from_slice(&block[8..]);
        }
    }
    let mut out = a.to_vec();
    for block in &r {
        out.extend_from_slice(block);
    }
    out
}

/// A GTK KDE for key index `id`.
pub fn gtk_kde(id: u8, gtk: &[u8; 16]) -> Vec<u8> {
    let mut kde = vec![0xDD, 22, 0x00, 0x0f, 0xac, 0x01, id, 0x00];
    kde.extend_from_slice(gtk);
    kde
}

/// An IGTK KDE for key index 4.
pub fn igtk_kde(igtk: &[u8; 16]) -> Vec<u8> {
    let mut kde = vec![0xDD, 28, 0x00, 0x0f, 0xac, 0x09, 0x04, 0x00, 1, 0, 0, 0, 0, 0];
    kde.extend_from_slice(igtk);
    kde
}

/// Pad key data to the key-wrap block size with 0xDD then zeros.
pub fn pad(mut kd: Vec<u8>) -> Vec<u8> {
    if !kd.len().is_multiple_of(8) || kd.len() < 16 {
        kd.push(0xDD);
        while !kd.len().is_multiple_of(8) || kd.len() < 16 {
            kd.push(0);
        }
    }
    kd
}

pub fn rc(n: u8) -> [u8; 8] {
    [0, 0, 0, 0, 0, 0, 0, n]
}

/// One EAPOL-Key frame from the AP under `kck` (an all-zero KCK for message 1,
/// which carries no MIC).
pub fn frame(akm: Akm, info: u16, replay: u8, nonce: &[u8; 32], kd: &[u8], kck: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; 99 + kd.len()];
    let f = KeyFrame { key_info: akm.key_version() | info, replay_counter: &rc(replay), nonce, key_data: kd };
    let n = build_key_frame_kind(&mut out, 2, akm.mic_kind(), &f, kck).unwrap();
    out.truncate(n);
    out
}

pub fn message1(akm: Akm, replay: u8, anonce: &[u8; 32]) -> Vec<u8> {
    frame(akm, KEY_INFO_PAIRWISE | KEY_INFO_ACK, replay, anonce, &[], &[0u8; 16])
}

/// Message 3 carrying `plain_kd`, wrapped under the PTK's KEK.
pub fn message3(akm: Akm, replay: u8, anonce: &[u8; 32], ptk: &[u8; 48], plain_kd: &[u8]) -> Vec<u8> {
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
pub fn group1(akm: Akm, replay: u8, ptk: &[u8; 48], plain_kd: &[u8]) -> Vec<u8> {
    let kek: [u8; 16] = ptk[16..32].try_into().unwrap();
    let wrapped = aes_wrap(&kek, &pad(plain_kd.to_vec()));
    let info = KEY_INFO_ACK | KEY_INFO_MIC | KEY_INFO_SECURE | KEY_INFO_ENCRYPTED;
    frame(akm, info, replay, &[0u8; 32], &wrapped, &ptk[..16])
}
