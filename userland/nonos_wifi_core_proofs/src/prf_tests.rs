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

//! The 802.11 PRF (IEEE Std 802.11-2020, 12.7.1.2: HMAC-SHA1 over the
//! label, a zero octet, the data and a counter octet) against the PRF test
//! vectors of Annex J.3. Case 1 is checked in full (512 bits); case 2 over
//! its first 256 bits.

use nonos_wifi_core::wpa::prf::prf;

fn h(s: &str) -> Vec<u8> {
    (0..s.len() / 2).map(|i| u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()).collect()
}

#[test]
fn prf_case_1() {
    let mut out = [0u8; 64];
    prf(&[0x0b; 20], b"prefix", b"Hi There", &mut out);
    let want = "bcd4c650b30b9684951829e0d75f9d54b862175ed9f00606e17d8da35402ffee\
                75df78c3d31e0f889f012120c0862beb67753e7439ae242edb8373698356cf5a";
    assert_eq!(out.to_vec(), h(want));
}

#[test]
fn prf_case_2() {
    let mut out = [0u8; 64];
    prf(b"Jefe", b"prefix-2", b"what do ya want for nothing?", &mut out);
    let want = "47c4908e30c947521ad20be9053450ecbea23d3aa604b77326d8b3825ff7475c";
    assert_eq!(out[..32].to_vec(), h(want));
}

#[test]
fn a_shorter_output_is_a_prefix_of_a_longer_one() {
    let (mut a, mut b) = ([0u8; 16], [0u8; 48]);
    prf(b"k", b"label", b"data", &mut a);
    prf(b"k", b"label", b"data", &mut b);
    assert_eq!(a[..], b[..16]);
}
