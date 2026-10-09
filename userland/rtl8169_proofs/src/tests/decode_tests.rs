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

//! The request header is another capsule's bytes: decoding it must never
//! panic, must refuse a short or mistagged header, and must read each field
//! from where the wire format puts it.

use crate::protocol::{decode_request, HDR_LEN};

const MAGIC: [u8; 4] = 0x4E4E_4554u32.to_le_bytes();

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

#[test]
fn a_short_or_mistagged_header_is_refused() {
    let mut h = [0u8; HDR_LEN];
    h[0..4].copy_from_slice(&MAGIC);
    h[4..6].copy_from_slice(&1u16.to_le_bytes());
    for len in 0..HDR_LEN {
        assert!(decode_request(&h[..len]).is_none(), "{len} bytes decoded");
    }
    assert!(decode_request(&h).is_some());
    let mut bad = h;
    bad[3] ^= 0x80;
    assert!(decode_request(&bad).is_none(), "wrong magic");
    let mut bad = h;
    bad[4] = 9;
    assert!(decode_request(&bad).is_none(), "wrong version");
}

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    for seed in 1..50_000u64 {
        let mut s = seed;
        let len = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC);
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}
