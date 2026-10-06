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

//! Every frame a client can put in net.dns's inbox, through the real header
//! decode and, when it is refused, the reply the loop sends for it. Any
//! process holding the endpoint can send any bytes, so each one must either
//! decode to the request and body it names or draw exactly one reply naming
//! the op and request id it carried: the caller is blocked in its call until
//! that reply comes.

use nonos_libc::{serial, take_replies};

use crate::protocol::{E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION, MAGIC};
use crate::server::parse_req::{parse, refused, HDR_LEN};
use crate::server::respond::respond;

const FUZZ_ROUNDS: usize = 200_000;
/// The receive buffer the loop hands parse a slice of.
const RX_LEN: usize = HDR_LEN + crate::protocol::IPC_PAYLOAD_MAX;
/// The last op the service knows.
const LAST_OP: u16 = crate::protocol::OP_SET_UPSTREAM;
const SENDER: u32 = 31;

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
    f.extend_from_slice(&1u16.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0, 0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
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

/// What must hold for any input. Returns the refusal's status, or None when
/// the frame is served.
fn check(buf: &[u8]) -> Option<u16> {
    let errno = match parse(buf) {
        Ok((req, body)) => {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!((le32(buf, 0), le16(buf, 4)), (MAGIC, 1));
            assert_eq!((req.op, req.request_id), (le16(buf, 6), le32(buf, 12)));
            let len = le32(buf, 16) as usize;
            assert_eq!(body, &buf[HDR_LEN..HDR_LEN + len], "the body is what the length names");
            return None;
        }
        Err(errno) => errno,
    };
    assert!([E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION].contains(&errno), "status {errno}");
    let req = refused(buf);
    if buf.len() < HDR_LEN {
        assert_eq!(errno, E_BAD_LEN);
        assert_eq!((req.op, req.request_id), (0, 0));
    } else {
        assert_eq!((req.op, req.request_id), (le16(buf, 6), le32(buf, 12)));
    }
    // The loop's refusal arm: refused, then respond with no body.
    let _ = take_replies();
    let mut tx = vec![0u8; RX_LEN];
    assert!(respond(SENDER, req.op, errno, req.request_id, 0, &mut tx) >= 0);
    let replies = take_replies();
    assert_eq!(replies.len(), 1, "one reply for a refused frame of {} bytes", buf.len());
    let reply = &replies[0];
    assert_eq!((reply.pid, reply.bytes.len()), (SENDER, HDR_LEN));
    assert_eq!((le32(&reply.bytes, 0), le16(&reply.bytes, 4)), (MAGIC, 1));
    assert_eq!((le16(&reply.bytes, 6), le16(&reply.bytes, 8)), (req.op, errno));
    assert_eq!((le32(&reply.bytes, 12), le32(&reply.bytes, 16)), (req.request_id, 0));
    Some(errno)
}

#[test]
fn random_frames_are_served_or_answered_with_a_refusal() {
    let _guard = serial();
    let mut s = 0x4E44_4E53_0000_0001u64;
    let (mut served, mut refused_n) = (0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (RX_LEN - HDR_LEN + 1) as u64) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).saturating_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % (u64::from(LAST_OP) + 2)) as u16,
        };
        let mut f = frame(op, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        match check(&f) {
            None => served += 1,
            Some(_) => refused_n += 1,
        }
    }
    assert!(served > FUZZ_ROUNDS / 4, "the generator reaches the dispatch: {served}");
    assert!(refused_n > FUZZ_ROUNDS / 4, "the generator reaches the refusals: {refused_n}");
}

#[test]
fn boundary_frames() {
    let _guard = serial();
    // Empty, and one byte short of a header: refused under zeros.
    let short = frame(1, 7, 0, &[]);
    for f in [&[][..], &short[..HDR_LEN - 1]] {
        assert_eq!(check(f), Some(E_BAD_LEN), "{} bytes", f.len());
    }
    // A bare header is served with an empty body.
    assert_eq!(check(&short), None);
    // A length field one larger than the bytes is refused; one smaller is
    // served with the body it names.
    assert_eq!(check(&frame(1, 9, 5, &[1, 2, 3, 4])), Some(E_BAD_LEN));
    assert_eq!(check(&frame(1, 9, 3, &[1, 2, 3, 4])), None);
    assert_eq!(parse(&frame(1, 9, 3, &[1, 2, 3, 4])).ok().map(|(_, b)| b.len()), Some(3));
    // The length field at its maximum does not wrap into a short frame.
    assert_eq!(check(&frame(1, 11, u32::MAX, &[])), Some(E_BAD_LEN));
    // Every op, one past the last and the largest decode as they came.
    for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
        assert_eq!(check(&frame(op, u32::from(op), 0, &[])), None, "op {op}");
    }
    // The largest frame the receive buffer holds is served whole.
    let big = vec![0x5Au8; RX_LEN - HDR_LEN];
    let f = frame(1, 13, big.len() as u32, &big);
    assert_eq!(check(&f), None);
    assert_eq!(parse(&f).ok().map(|(_, b)| b.len()), Some(RX_LEN - HDR_LEN));
    // Wrong magic and wrong version are answered under the request they carry.
    let mut f = frame(2, 15, 0, &[]);
    f[0] ^= 0xFF;
    assert_eq!(check(&f), Some(E_BAD_MAGIC));
    let mut f = frame(2, 17, 0, &[]);
    f[4] = 2;
    assert_eq!(check(&f), Some(E_BAD_VERSION));
}
