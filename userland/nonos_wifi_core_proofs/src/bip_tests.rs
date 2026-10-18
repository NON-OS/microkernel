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

//! BIP-CMAC-128 and the MMIE on their own. The vector is a broadcast deauthentication under a fixed IGTK, key index 4,
//! IPN 4; its MIC was computed independently with the Python `cryptography`
//! AES-CMAC (version 50) over the AAD and body as 12.5.4.4 lays them out.
//! AES-CMAC itself is checked against RFC 4493 in `crypto_tests`.

use nonos_wifi_core::pmf::{bip_protect, bip_verify, Mmie};

const IGTK: [u8; 16] = [
    0x4e, 0xa9, 0x54, 0x3e, 0x09, 0xcf, 0x2b, 0x1e, 0xca, 0x66, 0xff, 0xc5, 0x8b, 0xde, 0xcb, 0xcf,
];
// Deauthentication to broadcast from 02:00:00:00:00:00, sequence 0x0002, reason 2.
const DEAUTH: [u8; 26] = [
    0xc0, 0, 0, 0, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0x02,
    0x00, 0x02, 0x00,
];
const MMIE: [u8; 18] =
    [76, 16, 4, 0, 4, 0, 0, 0, 0, 0, 0x48, 0xdf, 0xbf, 0xa7, 0xb8, 0x27, 0x88, 0x72];

#[test]
fn bip_signs_the_reference_deauthentication() {
    let f = bip_protect(&DEAUTH, 4, 4, &IGTK).unwrap();
    assert_eq!(&f[..26], &DEAUTH[..]);
    assert_eq!(&f[26..], &MMIE[..]);
    assert_eq!(
        bip_verify(&f, &IGTK),
        Some(Mmie { key_id: 4, ipn: 4, mic: MMIE[10..].try_into().unwrap() })
    );
}

#[test]
fn bip_ignores_what_changes_in_flight_and_nothing_else() {
    let f = bip_protect(&DEAUTH, 4, 4, &IGTK).unwrap();
    let mut retried = f.clone();
    retried[1] |= 0x08 | 0x10 | 0x20; // Retry, Power Management, More Data
    retried[22] = 0x13; // the sequence control is outside the AAD
    assert!(bip_verify(&retried, &IGTK).is_some());
    for at in [0, 4, 10, 16, 24, 28, 36] {
        let mut bad = f.clone();
        bad[at] ^= 0x01;
        assert!(bip_verify(&bad, &IGTK).is_none(), "octet {at}");
    }
    assert!(bip_verify(&f, &[0u8; 16]).is_none(), "another key");
    assert!(bip_verify(&DEAUTH, &IGTK).is_none(), "no MMIE");
    let mut trailing = f.clone();
    trailing.extend_from_slice(&[221, 0]);
    assert!(bip_verify(&trailing, &IGTK).is_none(), "the MMIE must come last");
}
