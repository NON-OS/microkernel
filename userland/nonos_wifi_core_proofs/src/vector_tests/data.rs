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
//! The CCMP data frame around the RFC 3610 cipher: the nonce and the AAD
//! byte for byte against the 802.11 layout (so a construction error cannot
//! hide behind a self-consistent round trip), an ethernet frame through
//! encapsulation and back, and a flipped MIC or ciphertext byte refused.

use nonos_wifi_core::dot11::data::{build_aad, build_nonce, decap, encap};

const OUR: [u8; 6] = [0x02, 0, 0, 0, 0, 0x02];
const BSSID: [u8; 6] = [0x02, 0, 0, 0, 0, 0x01];
const DST: [u8; 6] = [0x33, 0x33, 0, 0, 0, 0x16];
const TK: [u8; 16] = [
    0xc9, 0x7c, 0x1f, 0x67, 0xce, 0x37, 0x11, 0x85, 0x51, 0x4a, 0x8a, 0x19, 0xf2, 0xbd, 0xd5, 0x2f,
];

#[test]
fn the_nonce_and_aad_follow_the_ieee_layout() {
    // Data to the DS, protected; sequence 5, which the AAD masks out.
    let (fc, sc) = (0x4108u16, 5u16 << 4);
    let mut want = [0u8; 22];
    want[..2].copy_from_slice(&fc.to_le_bytes());
    want[2..8].copy_from_slice(&BSSID);
    want[8..14].copy_from_slice(&OUR);
    want[14..20].copy_from_slice(&DST);
    assert_eq!(build_aad(fc, &BSSID, &OUR, &DST, sc), want);
    // Priority, the transmitter, then the packet number most significant first.
    let want = [0, 0x02, 0, 0, 0, 0, 0x02, 0, 0, 0, 0x01, 0x02, 0x03];
    assert_eq!(build_nonce(&OUR, 0x0001_0203), want);
}

#[test]
fn an_ethernet_frame_survives_ccmp_and_a_flipped_byte_does_not() {
    let eth = [&DST[..], &OUR, &[0x08, 0x00], b"hello over the air"].concat();
    let mut frame = encap(&eth, OUR, BSSID, 7, &TK, 7).unwrap();
    assert_ne!(&frame[32..32 + eth.len() - 14], &eth[14..], "the payload is encrypted");
    assert_eq!(decap(&frame, &TK).unwrap(), eth);
    for at in [frame.len() - 1, 40] {
        frame[at] ^= 0x80;
        assert!(decap(&frame, &TK).is_none(), "byte {at} flipped");
        frame[at] ^= 0x80;
    }
}
