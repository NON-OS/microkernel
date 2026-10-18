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

//! Transmit-side CCMP: `dot11::ccmp::encrypt`. The station's `protect_mgmt`,
//! which uses it for the deauthentication a station sends when management
//! frame protection is on, is in `protect_mgmt_tests`.
//!
//! The management frame vector is IEEE Std 802.11-2012 M.9.2 (CCMP with a
//! unicast Deauthentication frame); its input bytes were copied from hostapd's
//! transcription in wlantest/test_vectors.c test_vector_ccmp_mgmt()
//! (https://w1.fi/cgit/hostap/plain/wlantest/test_vectors.c, sha256
//! 9aceafbe03d32ca537dab030de0350c5d9cc658de176bc21c76ab95b7e30a633). That
//! copy carries no expected output, so the expected MPDU was computed
//! independently with the Python `cryptography` AES-CCM under the
//! 802.11-2020 12.5.3.3 rules for a management frame (Management flag 0x10 in
//! the nonce; Retry, Power Management and More Data masked out of the frame
//! control in the AAD, the subtype kept).

use crate::hex::h;
use nonos_wifi_core::dot11::ccmp::{decrypt, encrypt};
use nonos_wifi_core::dot11::data::{build_data, protect};

const TK_M92: &str = "66ed21042f9f26d7115706e40414cf2e";
const M92_PLAIN: &str = "c0000000020000000100020000000000020000000000600002 00";
const M92_CIPHER: &str =
    "c0400000020000000100020000000000020000000000600001000020000000001d07cafd0409bb8bafef";

pub const STA: [u8; 6] = [0x02, 0x11, 0x22, 0x33, 0x44, 0x55];
pub const AP: [u8; 6] = [0x02, 0xAA, 0xBB, 0xCC, 0xDD, 0xEE];

pub fn tk() -> [u8; 16] {
    h(TK_M92).try_into().unwrap()
}

#[test]
fn a_deauthentication_is_protected_as_the_annex_vector_lays_out() {
    let enc = encrypt(&h(M92_PLAIN), 1, &tk()).expect("a well formed frame");
    assert_eq!(enc, h(M92_CIPHER), "IEEE 802.11-2012 M.9.2");
    assert_eq!(decrypt(&enc, &tk()).expect("the MIC verifies"), vec![0x02, 0x00]);
}

#[test]
fn encrypt_matches_the_data_path_for_data_frames() {
    let eth = [&AP[..], &STA[..], &[0x08, 0x00], b"payload"].concat();
    let mpdu = build_data(&eth, STA, AP, 7).expect("framed");
    assert_eq!(encrypt(&mpdu, 3, &tk()), protect(&mpdu, 3, &tk()), "one CCMP for both");
}

#[test]
fn a_runt_frame_is_not_protected() {
    assert!(encrypt(&[0xC0, 0x00, 0x00], 1, &tk()).is_none());
}
