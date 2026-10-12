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

//! The wire header, as in the other driver proofs: decoding is total, reads
//! every field from its offset, and an encoded header decodes back.

use super::fuzz::{rounds, xorshift};
use crate::protocol::{decode_request, encode_response_header, Request, HDR_LEN};

#[test]
fn decode_never_panics_and_reads_fields_from_their_offsets() {
    const MAGIC: u32 = 0x4E4E_4554; // the wire tag from protocol/header.rs
    for seed in 1..rounds(100_000) {
        let mut s = seed;
        let blen = (xorshift(&mut s) % 40) as usize;
        let mut buf: Vec<u8> = (0..blen).map(|_| (xorshift(&mut s) & 0xff) as u8).collect();
        if buf.len() >= 6 && seed % 2 == 0 {
            buf[0..4].copy_from_slice(&MAGIC.to_le_bytes());
            buf[4..6].copy_from_slice(&1u16.to_le_bytes());
        }
        if let Some(req) = decode_request(&buf) {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(req.op, u16::from_le_bytes([buf[6], buf[7]]));
            assert_eq!(req.flags, u16::from_le_bytes([buf[8], buf[9]]));
            assert_eq!(req.request_id, u32::from_le_bytes([buf[12], buf[13], buf[14], buf[15]]));
            assert_eq!(req.payload_len, u32::from_le_bytes([buf[16], buf[17], buf[18], buf[19]]));
        }
    }
}

#[test]
fn encoded_response_headers_decode_back_to_the_request_fields() {
    for seed in 1..rounds(20_000) {
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
