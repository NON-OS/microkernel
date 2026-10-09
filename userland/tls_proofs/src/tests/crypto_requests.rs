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

//! Any request a caller can send the crypto pool, through its real decode and
//! dispatch, which the shim already hosts to answer the TLS client. The TLS
//! client, the browser and model fetch call the pool directly, so its bodies
//! are anyone's. Every frame that decodes draws exactly one reply from the
//! dispatch, naming the op, flags and request id it answers, with the status
//! word first and the length field the rest of the reply; every frame that
//! does not is answered by the loop with a zero header and EINVAL. Bodies are
//! biased toward the shapes the handlers read: keys, nonces, signatures and
//! the small length words that frame them.

use nonos_libc::protocol::{
    decode_request, encode_response, EBADMSG, EINVAL, EIO, EMSGSIZE, HDR_LEN, MAX_PAYLOAD_BYTES,
    OP_HEALTHCHECK, OP_RSA_VERIFY,
};
use nonos_libc::server::dispatch::dispatch;

const MAGIC: u32 = 0x4E4F_4358;
/// The sizes the handlers read: keys, nonces, tags, points and signatures.
const SHAPES: [usize; 18] = [0, 1, 12, 16, 31, 32, 33, 44, 48, 64, 65, 96, 97, 128, 161, 241, 256, 512];

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn le16(b: &[u8], off: usize) -> u16 {
    u16::from_le_bytes([b[off], b[off + 1]])
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn frame(op: u16, flags: u16, request_id: u32, body: &[u8]) -> Vec<u8> {
    let mut f = MAGIC.to_le_bytes().to_vec();
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&(body.len() as u32).to_le_bytes());
    f.extend_from_slice(body);
    f
}

fn body(s: &mut u64) -> Vec<u8> {
    let parts = 1 + xorshift(s) % 4;
    let mut b = Vec::new();
    for _ in 0..parts {
        let n = match xorshift(s) % 4 {
            0 => (xorshift(s) % 600) as usize,
            _ => SHAPES[(xorshift(s) % SHAPES.len() as u64) as usize],
        };
        b.extend((0..n).map(|_| xorshift(s) as u8));
    }
    // Length words where the framed handlers keep them.
    for _ in 0..xorshift(s) % 3 {
        if b.len() < 4 {
            break;
        }
        let at = (xorshift(s) % (b.len() as u64 - 3)) as usize & !3;
        let v: u32 = match xorshift(s) % 4 {
            0 => (xorshift(s) % 64) as u32,
            1 => b.len() as u32 - (xorshift(s) % 4) as u32,
            2 => u32::MAX - (xorshift(s) % 4) as u32,
            _ => (xorshift(s) % 600) as u32,
        };
        b[at..at + 4].copy_from_slice(&v.to_le_bytes());
    }
    b
}

/// What must hold for any frame. Returns the reply's status.
fn check(f: &[u8]) -> i32 {
    let reply = match decode_request(f) {
        Ok(req) => {
            let (op, flags, id) = (req.op, req.flags, req.request_id);
            let reply = dispatch(req);
            assert_eq!((le16(&reply, 6), le16(&reply, 8), le32(&reply, 12)), (op, flags, id));
            reply
        }
        Err(_) => encode_response(0, 0, 0, EINVAL, &[]),
    };
    assert!(reply.len() >= HDR_LEN + 4, "a header and a status");
    assert_eq!((le32(&reply, 0), le16(&reply, 4)), (MAGIC, 1));
    assert_eq!(le32(&reply, 16) as usize, reply.len() - HDR_LEN, "the length is the rest");
    let status = le32(&reply, HDR_LEN) as i32;
    assert!([0, EINVAL, EBADMSG, EIO, EMSGSIZE].contains(&status), "status {status}");
    if status != 0 {
        assert_eq!(reply.len(), HDR_LEN + 4, "a refusal carries nothing past its status");
    }
    status
}

#[test]
fn any_request_draws_one_whole_reply() {
    let mut s = 0x4E4F_4358_0000_0001u64;
    let mut served = 0usize;
    for round in 0..200_000u32 {
        let r = xorshift(&mut s);
        let op = match r % 8 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % u64::from(OP_RSA_VERIFY + 2)) as u16,
        };
        let mut f = frame(op, xorshift(&mut s) as u16, round, &body(&mut s));
        match (r >> 8) % 16 {
            0 => f.truncate((xorshift(&mut s) % (f.len() as u64 + 1)) as usize),
            1 => f[(xorshift(&mut s) % 20) as usize] ^= 1 << (xorshift(&mut s) % 8),
            _ => {}
        }
        if check(&f) == 0 {
            served += 1;
        }
    }
    assert!(served > 20_000, "the generator reaches the handlers' success paths: {served}");
}

#[test]
fn boundary_requests() {
    assert_eq!(check(&[]), EINVAL);
    let f = frame(OP_HEALTHCHECK, 0, 1, &[]);
    assert_eq!(check(&f[..HDR_LEN - 1]), EINVAL, "one byte short of the header");
    assert_eq!(check(&f), 0, "a bare healthcheck");
    // A length field past the bytes, and past the pool's limit.
    let mut over = frame(OP_HEALTHCHECK, 0, 2, &[1, 2, 3]);
    over[16..20].copy_from_slice(&4u32.to_le_bytes());
    assert_eq!(check(&over), EINVAL);
    let mut huge = frame(OP_HEALTHCHECK, 0, 3, &[]);
    huge[16..20].copy_from_slice(&(MAX_PAYLOAD_BYTES + 1).to_le_bytes());
    assert_eq!(check(&huge), EINVAL);
    // Every op with an empty body, with one byte, and with each handler shape.
    for op in (0..=OP_RSA_VERIFY + 1).chain([u16::MAX]) {
        for n in SHAPES {
            check(&frame(op, 7, u32::from(op), &vec![0x5A; n]));
        }
    }
}
