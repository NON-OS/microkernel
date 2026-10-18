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

//! The wire header: decoding refuses a short or mistagged one and reads
//! every field from its offset, and an encoded reply decodes back.

use super::frames::xorshift;
use crate::protocol::{decode_request, encode_response_header, Request, HDR_LEN};

const MAGIC: u32 = 0x4E42_4C4B; // "NBLK", the wire tag from protocol/header.rs
const VERSION: u16 = 1;

#[test]
fn decode_rejects_short_and_mistagged_headers_and_reads_fields_faithfully() {
    for len in 0..HDR_LEN {
        assert!(decode_request(&vec![0xa5u8; len]).is_none());
    }
    let mut hdr = [0u8; HDR_LEN];
    hdr[0..4].copy_from_slice(&MAGIC.to_le_bytes());
    hdr[4..6].copy_from_slice(&VERSION.to_le_bytes());
    hdr[6..8].copy_from_slice(&4u16.to_le_bytes());
    hdr[16..20].copy_from_slice(&512u32.to_le_bytes());
    let r = decode_request(&hdr).expect("a tagged header decodes");
    assert_eq!((r.op, r.payload_len), (4, 512));
    let mut bad_magic = hdr;
    bad_magic[0] ^= 1;
    assert!(decode_request(&bad_magic).is_none());
    let mut bad_version = hdr;
    bad_version[4] ^= 1;
    assert!(decode_request(&bad_version).is_none());
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..20_000u64 {
        let mut s = seed;
        let request = Request {
            op: (xorshift(&mut s) & 0xffff) as u16,
            flags: (xorshift(&mut s) & 0xffff) as u16,
            request_id: (xorshift(&mut s) & 0xffff_ffff) as u32,
            payload_len: 0,
        };
        let payload_len = (xorshift(&mut s) & 0xffff_ffff) as u32;
        let mut out = [0u8; HDR_LEN];
        encode_response_header(&mut out, &request, payload_len);
        let back = decode_request(&out).expect("a response header carries the wire tag");
        assert_eq!(back.op, request.op);
        assert_eq!(back.flags, request.flags);
        assert_eq!(back.request_id, request.request_id);
        assert_eq!(back.payload_len, payload_len);
    }
}
