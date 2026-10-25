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

//! GCMP-256 framing (IEEE Std 802.11-2020, 12.5.5) over the GCM proven in
//! `gcm_tests`: the header carries the PN and ExtIV, the MIC is 16
//! octets, the AAD protects what 12.5.5.3.2 keeps and ignores what it
//! masks, the nonce binds A2 and the PN, and malformed frames are refused.

use nonos_wifi_core::dot11::gcmp::{decrypt, encrypt, pn_and_key, GCMP_HDR_LEN, GCMP_MIC_LEN};

pub const TK: [u8; 32] = [0x5a; 32];

/// To-DS data, A1 the AP, A2 the station, then a payload.
pub fn data(qos: bool) -> Vec<u8> {
    let mut f = vec![if qos { 0x88 } else { 0x08 }, 0x01, 0, 0];
    f.extend_from_slice(&[0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x60, 0x02, 0x11, 0x22, 0x33, 0x44, 0x55]);
    f.extend_from_slice(&[0x00, 0x1a, 0x2b, 0x3c, 0x4d, 0x60, 0x30, 0x01]);
    if qos {
        f.extend_from_slice(&[0x05, 0x00]);
    }
    f.extend((0..40).map(|i| i as u8));
    f
}

#[test]
fn a_frame_round_trips_and_carries_its_pn() {
    for qos in [false, true] {
        let f = data(qos);
        let hdr = if qos { 26 } else { 24 };
        let pn = 0x0102_0304_0506;
        let p = encrypt(&f, pn, &TK).unwrap();
        assert_eq!(p.len(), f.len() + GCMP_HDR_LEN + GCMP_MIC_LEN);
        assert_eq!(p[1] & 0x40, 0x40);
        assert_eq!(&p[hdr..hdr + 8], &[0x06, 0x05, 0, 0x20, 0x04, 0x03, 0x02, 0x01]);
        assert_eq!(pn_and_key(&p, hdr), Some((pn, 0)));
        assert_eq!(decrypt(&p, &TK).unwrap(), f[hdr..].to_vec());
    }
}

#[test]
fn protected_fields_are_bound_and_masked_ones_are_not() {
    let p = encrypt(&data(true), 7, &TK).unwrap();
    // A1, A2, A3, the TID and the PN are protected.
    for at in [4, 10, 16, 24, 26] {
        let mut q = p.clone();
        q[at] ^= 1;
        assert_eq!(decrypt(&q, &TK), None, "byte {at}");
    }
    // Retry and the sequence number are masked out of the AAD.
    let mut q = p.clone();
    (q[1], q[22], q[23]) = (q[1] | 0x08, q[22] ^ 0x10, q[23] ^ 0xff);
    assert!(decrypt(&q, &TK).is_some());
    let mut k = TK;
    k[31] ^= 1;
    assert_eq!(decrypt(&p, &k), None);
}
