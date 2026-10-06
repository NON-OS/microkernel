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

//! Every frame a caller can send ramfs, through its real decode. A frame
//! it decodes reaches the dispatch, which returns a reply on every arm; a
//! frame it refuses is answered by the loop with `encode_response(0, EINVAL,
//! &[])`. These pin which frames decode, that the decode takes the sequence
//! number, op and body the frame carries without reading past it, and the
//! refusal's bytes.

use crate::ramfs::{decode_request, encode_response, EINVAL};

/// The receive buffer the loop hands the decode a slice of.
const MAX_MSG: usize = 8192;
/// The last op the service knows.
const LAST_OP: u16 = crate::ramfs::OP_TRUNCATE;
/// Whether the decode also refuses a frame whose bytes 6 and 7 are not zero.
const RESERVED_CHECKED: bool = true;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(seq: u32, op: u16, reserved: u16, body: &[u8]) -> Vec<u8> {
    let mut f = seq.to_le_bytes().to_vec();
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&reserved.to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// What must hold for any frame. Returns whether it decoded.
fn check(f: &[u8]) -> bool {
    let reserved_ok = !RESERVED_CHECKED || f.get(6..8) == Some(&[0, 0][..]);
    let Some(req) = decode_request(f) else {
        assert!(f.len() < 8 || !reserved_ok, "a whole header of {} bytes refused", f.len());
        let refusal = encode_response(0, EINVAL, &[]);
        assert_eq!(refusal, [0, 0, 0, 0, 0xEA, 0xFF, 0xFF, 0xFF], "the refusal is seq 0, EINVAL");
        return false;
    };
    assert!(f.len() >= 8 && reserved_ok);
    assert_eq!(req.seq, u32::from_le_bytes([f[0], f[1], f[2], f[3]]));
    assert_eq!(req.op, u16::from_le_bytes([f[4], f[5]]));
    assert_eq!(req.payload, &f[8..], "the body is the rest of the frame");
    true
}

#[test]
fn any_frame_decodes_or_is_refused() {
    let mut s = 0x5241_4D46_0000_0001u64;
    let mut decoded = 0usize;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let body: Vec<u8> = (0..xorshift(&mut s) % 96).map(|_| xorshift(&mut s) as u8).collect();
        let op = match r % 4 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % (u64::from(LAST_OP) + 2)) as u16,
        };
        let reserved = if (r >> 8) & 7 == 0 { xorshift(&mut s) as u16 } else { 0 };
        let mut f = frame(xorshift(&mut s) as u32, op, reserved, &body);
        if (r >> 12) & 7 == 0 {
            f.truncate((xorshift(&mut s) % 9) as usize);
        }
        if check(&f) {
            decoded += 1;
        }
    }
    assert!(decoded > 100_000, "the generator reaches the dispatch: {decoded}");
}

#[test]
fn boundary_frames() {
    assert!(!check(&[]));
    let f = frame(7, 1, 0, &[]);
    assert!(!check(&f[..7]), "one byte short of the header");
    assert!(check(&f), "a bare header");
    for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
        assert!(check(&frame(u32::from(op), op, 0, &[1, 2, 3])), "op {op}");
    }
    assert!(check(&frame(u32::MAX, 1, 0, &vec![0xA5; MAX_MSG - 8])), "the largest frame");
    assert_eq!(check(&frame(9, 1, 1, &[])), !RESERVED_CHECKED);
}
