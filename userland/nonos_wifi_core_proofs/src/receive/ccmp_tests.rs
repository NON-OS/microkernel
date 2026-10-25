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

//! CCMP on the receive half of an associated link is checked against IEEE
//! Std 802.11-2012 M.6.4 (the CCMP test vector: input bytes copied from
//! hostapd's transcription in wlantest/test_vectors.c test_vector_ccmp(),
//! fetched from
//! https://w1.fi/cgit/hostap/plain/wlantest/test_vectors.c, sha256
//! 9aceafbe03d32ca537dab030de0350c5d9cc658de176bc21c76ab95b7e30a633; the
//! expected encrypted MPDU is the published one). The QoS vector was computed
//! independently with the Python `cryptography` AES-CCM and the 802.11-2020
//! 12.5.3.3.3 and 12.5.3.3.4 rules (TID in the nonce, QoS Control with only
//! the TID in the AAD), since the Annex vector is non-QoS and access points
//! send QoS data.

use crate::hex::h;
use nonos_wifi_core::dot11::ccmp::{decrypt, view};
use nonos_wifi_core::dot11::data::protect;

const TK_M64: &str = "c97c1f67ce371185514a8a19f2bdd52f";
const M64_PLAIN: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    f8ba1a55d02f85ae967bb62fb6cda8eb7e78a050";
const M64_CIPHER: &str = "0848c32c0fd2e128a57c5030f1844408abaea5b8fcba8033
    0ce7002076970 3b5 f3d0a2fe9a3dbf2342a643e43246e80c3c04d0197845ce0b16f97623";
const QOS_CIPHER: &str = "88420000020000000002020000000001020000000077301225000201002000000000
    be61194719d3a6a5a42a2593b793f93ec4bb74166420a43f7a";

pub fn tk() -> [u8; 16] {
    h(TK_M64).try_into().unwrap()
}

#[test]
fn ccmp_matches_the_annex_m64_vector() {
    let enc = protect(&h(M64_PLAIN), 0xB5039776E70C, &tk()).unwrap();
    assert_eq!(enc, h(M64_CIPHER), "encrypted MPDU");
    assert_eq!(decrypt(&enc, &tk()).unwrap(), h(M64_PLAIN)[24..].to_vec(), "decrypts back");
}

#[test]
fn a_qos_frame_decrypts_with_its_tid_in_nonce_and_aad() {
    let f = h(QOS_CIPHER);
    let v = view(&f).unwrap();
    assert_eq!((v.hdr_len, v.tid), (26, 5));
    let plain = decrypt(&f, &tk()).expect("the QoS frame decrypts");
    assert_eq!(&plain[..8], &h("aaaa030000000800")[..]);
    assert_eq!(&plain[8..], b"hello qos");
    let mut flipped = f.clone();
    flipped[24] ^= 0x01; // another TID: a different nonce and AAD
    assert!(decrypt(&flipped, &tk()).is_none(), "the TID is authenticated");
}
