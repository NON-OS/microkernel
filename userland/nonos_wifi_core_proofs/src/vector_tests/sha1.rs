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
//! SHA-1 (RFC 3174), HMAC-SHA1 (RFC 2202), PBKDF2 (RFC 6070) and the PMK
//! (IEEE Std 802.11i Annex H.4). A correct PMK, 4096 PBKDF2 rounds over
//! HMAC-SHA1 over SHA-1, exercises the whole stack.

use nonos_wifi_core::wpa::hmac::hmac_sha1;
use nonos_wifi_core::wpa::pbkdf2::pbkdf2_sha1;
use nonos_wifi_core::wpa::ptk::{pmk, ptk};
use nonos_wifi_core::wpa::sha1::sha1;

pub(super) fn h(s: &str) -> Vec<u8> {
    (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
}

#[test]
fn sha1_hmac_and_pbkdf2_match_their_rfc_vectors() {
    assert_eq!(sha1(b"abc").to_vec(), h("a9993e364706816aba3e25717850c26c9cd0d89d"));
    assert_eq!(sha1(b"").to_vec(), h("da39a3ee5e6b4b0d3255bfef95601890afd80709"));
    let long = b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq";
    assert_eq!(sha1(long).to_vec(), h("84983e441c3bd26ebaae4aa1f95129e5e54670f1"));
    let mac = hmac_sha1(&[0x0b; 20], b"Hi There");
    assert_eq!(mac.to_vec(), h("b617318655057264e28bc0b6fb378c8ef146be00"));
    let mac = hmac_sha1(b"Jefe", b"what do ya want for nothing?");
    assert_eq!(mac.to_vec(), h("effcdf6ae5eb2fa2d27416d5f184df9c259a7c79"));
    let mut o = [0u8; 20];
    pbkdf2_sha1(b"password", b"salt", 1, &mut o);
    assert_eq!(o.to_vec(), h("0c60c80f961f0e71f3a9b524af6012062fe037a6"));
    pbkdf2_sha1(b"password", b"salt", 4096, &mut o);
    assert_eq!(o.to_vec(), h("4b007901b765489abead49d926f721d065a429c1"));
}

#[test]
fn the_pmk_matches_ieee_80211i_and_the_ptk_ignores_who_is_who() {
    let a = pmk(b"password", b"IEEE");
    assert_eq!(a.to_vec(), h("f42c6fc52df0ebef9ebb4b90b38a5f902e83fe1b135a70e23aed762e9710a12e"));
    let b = pmk(b"ThisIsAPassword", b"ThisIsASSID");
    assert_eq!(b.to_vec(), h("0dc0d6eb90555ed6419756b9a15ec3e3209b63df707dd508d14581f8982721af"));
    let (aa, spa) = ([0x00, 0x11, 0x22, 0x33, 0x44, 0x55], [0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb]);
    let (anonce, snonce) = ([0x11u8; 32], [0x22u8; 32]);
    let p = ptk(&a, &aa, &spa, &anonce, &snonce);
    assert_eq!(p, ptk(&a, &spa, &aa, &snonce, &anonce), "address and nonce order");
    assert_eq!(p, ptk(&a, &aa, &spa, &anonce, &snonce), "deterministic");
}
