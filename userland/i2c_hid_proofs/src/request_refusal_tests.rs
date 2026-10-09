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

//! Every frame a client can put in driver.i2c_hid0's inbox, through the driver's
//! real header decode and, when it is refused, the reply the loop now sends:
//! the driver's own encoder under `refused`, with E_INVAL. Each frame either
//! decodes to the request it carries or draws one whole reply naming the op
//! and request id it carried (zeros when it is too short to carry them), so
//! its caller is not left waiting out its timeout.

use crate::protocol::{parse, refused, response, E_INVAL, HDR_LEN};

const MAGIC: u32 = 0x4E48_4944;
const VERSION: u16 = 1;
/// The longest body the driver's receive buffer holds after the header.
const BODY_MAX: usize = crate::protocol::IPC_PAYLOAD_MAX;
/// The last op the driver knows.
const LAST_OP: u16 = crate::protocol::OP_DESCRIPTOR;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(op: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&u64::from(request_id).to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn le64(b: &[u8], off: usize) -> u64 {
    let mut a = [0u8; 8];
    a.copy_from_slice(&b[off..off + 8]);
    u64::from_le_bytes(a)
}

/// What must hold for any input. Returns whether it decoded.
fn check(buf: &[u8]) -> bool {
    if let Some((req, body)) = parse(buf) {
        assert!(buf.len() >= HDR_LEN && le32(buf, 0) == MAGIC && le16(buf, 4) == VERSION);
        assert_eq!((req.op, req.request_id), (le16(buf, 6), le64(buf, 8)));
        let len = le32(buf, 16) as usize;
        assert_eq!(body, &buf[HDR_LEN..HDR_LEN + len], "the body is what the length names");
        return true;
    }
    let req = refused(buf);
    let echo = if buf.len() >= HDR_LEN { (le16(buf, 6), le64(buf, 8)) } else { (0, 0) };
    assert_eq!((req.op, req.request_id), echo, "the refusal names its request");
    let mut tx = vec![0u8; HDR_LEN + BODY_MAX];
    let n = response(req.op, req.request_id, E_INVAL, &[], &mut tx);
    assert_eq!(n, HDR_LEN + 4, "a header and a status");
    let reply = &tx[..n];
    assert_eq!((le32(reply, 0), le16(reply, 4), le16(reply, 6)), (MAGIC, VERSION, req.op));
    assert_eq!((le64(reply, 8), le32(reply, 16)), (req.request_id, 4));
    assert_eq!(le32(reply, HDR_LEN) as i32, E_INVAL);
    false
}

#[test]
fn random_frames_are_served_or_answered_with_a_refusal() {
    let mut s = 0x4E48_4944_0000_0001u64;
    let (mut served, mut refused_n) = (0usize, 0usize);
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (BODY_MAX.min(512) as u64 + 8)) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).wrapping_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = (xorshift(&mut s) % (u64::from(LAST_OP) + 2)) as u16;
        let mut f = frame(op, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        if check(&f) {
            served += 1;
        } else {
            refused_n += 1;
        }
    }
    assert!(served > 50_000 && refused_n > 30_000, "both paths reached: {served} {refused_n}");
}

#[test]
fn boundary_frames() {
    assert!(!check(&[]));
    let short = frame(1, 7, 0, &[]);
    assert!(!check(&short[..HDR_LEN - 1]), "one byte short of a header");
    assert!(check(&short), "a bare header");
    assert!(!check(&frame(1, 9, 5, &[1, 2, 3, 4])), "a length field one past the bytes");
    assert!(check(&frame(1, 9, 3, &[1, 2, 3, 4])), "one short of them: the body it names");
    assert!(!check(&frame(1, 11, u32::MAX, &[])), "the largest length field");
    for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
        assert!(check(&frame(op, u32::from(op), 0, &[])), "op {op}");
    }
    let big = vec![0xA5u8; BODY_MAX];
    assert!(check(&frame(1, 13, BODY_MAX as u32, &big)), "the largest body");
    let mut bad = frame(1, 15, 0, &[]);
    bad[0] ^= 0xFF;
    assert!(!check(&bad), "the wrong magic");
    assert_eq!(refused(&bad).request_id, 15);
}
