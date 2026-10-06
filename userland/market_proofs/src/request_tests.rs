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

//! The market's request header decode and the length-prefixed readers its
//! OP_GET_APP, OP_GET_RELEASE and OP_INSTALL_READY bodies go through, with any
//! bytes. The header decode takes any frame without a panic and refuses what
//! is short or carries the wrong magic or version (the loop then answers with
//! a zero header and E_INVAL); the readers never read past the body, whatever
//! length a prefix claims.

#[path = "../../capsule_market/src/server/handlers/get_app/read_lp_string.rs"]
mod read_lp_string;
#[path = "../../capsule_market/src/server/handlers/get_release/take_lp.rs"]
mod take_lp;
#[path = "../../capsule_market/src/server/handlers/get_release/parse_pair.rs"]
mod parse_pair;

use alloc::vec::Vec;

use crate::protocol::{decode_request, HDR_LEN};
use parse_pair::parse_pair;
use read_lp_string::read_lp_string;
use take_lp::take_lp;

const MAGIC: u32 = 0x4E4D_4B54;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn frame(op: u16, request_id: u32, payload_len: u32) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0, 0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f
}

/// A length prefix: right, one over, one under, or anything.
fn lp(s: &mut u64, body: &[u8]) -> Vec<u8> {
    let n = body.len() as u32;
    let claimed = match xorshift(s) % 8 {
        0 => n + 1,
        1 => n.saturating_sub(1),
        2 => u32::MAX - (xorshift(s) % 4) as u32,
        3 => xorshift(s) as u32,
        _ => n,
    };
    let mut out = claimed.to_le_bytes().to_vec();
    out.extend_from_slice(body);
    out
}

#[test]
fn any_header_decodes_or_is_refused() {
    let mut s = 0x4E4D_4B54_0000_0002u64;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let mut f = frame(xorshift(&mut s) as u16, xorshift(&mut s) as u32, xorshift(&mut s) as u32);
        match r % 6 {
            0 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            1 => f[(xorshift(&mut s) % 6) as usize] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.extend((0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8)),
            _ => {}
        }
        match decode_request(&f) {
            Some(req) => {
                assert!(f.len() >= HDR_LEN);
                assert_eq!((le32(&f, 0), u16::from_le_bytes([f[4], f[5]])), (MAGIC, 1));
                assert_eq!((req.request_id, req.payload_len), (le32(&f, 12), le32(&f, 16)));
            }
            None => assert!(f.len() < HDR_LEN || le32(&f, 0) != MAGIC || f[4..6] != [1, 0]),
        }
    }
    assert!(decode_request(&[]).is_none());
    assert!(decode_request(&frame(1, 2, 0)[..HDR_LEN - 1]).is_none());
    assert!(decode_request(&frame(1, 2, u32::MAX)).is_some(), "the loop bounds the body");
}

#[test]
fn the_length_prefixed_readers_stay_inside_the_body() {
    let mut s = 0x4E4D_4B54_0000_0003u64;
    let mut paired = 0usize;
    for _ in 0..200_000 {
        let a: Vec<u8> = (0..xorshift(&mut s) % 24).map(|_| b'a' + (xorshift(&mut s) % 26) as u8).collect();
        let b: Vec<u8> = (0..xorshift(&mut s) % 24)
            .map(|_| match xorshift(&mut s) {
                r if r & 15 == 0 => r as u8,
                r => b'a' + (r % 26) as u8,
            })
            .collect();
        let mut body = lp(&mut s, &a);
        body.extend(lp(&mut s, &b));
        if xorshift(&mut s) & 3 == 0 {
            body.truncate((xorshift(&mut s) % (body.len() as u64 + 1)) as usize);
        }
        if let Some(name) = read_lp_string(&body) {
            assert_eq!(name.len(), le32(&body, 0) as usize);
            assert_eq!(name.as_bytes(), &body[4..4 + name.len()]);
        }
        if let Some((first, rest)) = take_lp(&body) {
            assert_eq!(4 + first.len() + rest.len(), body.len());
        }
        if let Some((x, y)) = parse_pair(&body) {
            assert!(4 + x.len() + 4 + y.len() <= body.len());
            paired += 1;
        }
    }
    assert!(paired > 20_000, "the generator reaches whole pairs: {paired}");
    assert!(read_lp_string(&[]).is_none() && take_lp(&[1, 0, 0]).is_none());
    assert!(read_lp_string(&[2, 0, 0, 0, b'a']).is_none(), "a prefix one past the bytes");
    assert_eq!(read_lp_string(&[1, 0, 0, 0, b'a', b'b']), Some("a"), "one short of them");
    assert!(take_lp(&[0xFF, 0xFF, 0xFF, 0xFF]).is_none(), "the largest prefix");
    assert!(read_lp_string(&[1, 0, 0, 0, 0xFF]).is_none(), "not UTF-8");
}
