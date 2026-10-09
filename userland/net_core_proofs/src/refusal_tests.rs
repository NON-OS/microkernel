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

//! Every frame a client can put in net.core's inbox, through the real header
//! decode and, when it is refused, the refusal the loop sends for it. Any
//! process holding the endpoint can send any bytes, so each one must either
//! decode to the request and body it names or draw exactly one reply naming
//! the magic, op and request id it carried (zeros when it is too short to
//! carry them): the caller is blocked in its call until that reply comes.

use nonos_libc::{serial, take_replies};

use crate::protocol::dns::MAGIC_NDNS;
use crate::protocol::errno::{E_BAD_LEN, E_BAD_VERSION};
use crate::protocol::ip::MAGIC_NIP4;
use crate::protocol::ops::{MAGIC_NDHC, MAGIC_NNET};
use crate::protocol::tcp::MAGIC_NTCP;
use crate::protocol::udp::MAGIC_NUDP;
use crate::server::parse_req::{parse, HDR_LEN, IPC_BUF_MAX};
use crate::server::refuse::refuse;

const FUZZ_ROUNDS: usize = 200_000;
/// The longest body the generator makes; the named set reaches the buffer's end.
const FUZZ_BODY_MAX: usize = 2048;
/// The last op any of the wires the loop serves knows (net.tcp's poll).
const LAST_OP: u16 = crate::protocol::tcp::OP_POLL;
const MAGICS: [u32; 6] = [MAGIC_NDNS, MAGIC_NIP4, MAGIC_NDHC, MAGIC_NNET, MAGIC_NTCP, MAGIC_NUDP];
const SENDER: u32 = 31;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(magic: u32, op: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&magic.to_le_bytes());
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
            assert_eq!(le16(buf, 4), 1);
            assert_eq!((req.magic, req.op, req.request_id), (le32(buf, 0), le16(buf, 6), le32(buf, 12)));
            let len = le32(buf, 16) as usize;
            assert_eq!(body, &buf[HDR_LEN..HDR_LEN + len], "the body is what the length names");
            return None;
        }
        Err(errno) => errno,
    };
    assert!([E_BAD_LEN, E_BAD_VERSION].contains(&errno), "status {errno}");
    // The loop's refusal arm.
    let _ = take_replies();
    let mut tx = vec![0u8; IPC_BUF_MAX];
    refuse(SENDER, buf, errno, &mut tx);
    let replies = take_replies();
    assert_eq!(replies.len(), 1, "one reply for a refused frame of {} bytes", buf.len());
    let reply = &replies[0];
    assert_eq!((reply.pid, reply.bytes.len()), (SENDER, HDR_LEN));
    let echo = if buf.len() >= HDR_LEN {
        (le32(buf, 0), le16(buf, 6), le32(buf, 12))
    } else {
        assert_eq!(errno, E_BAD_LEN);
        (0, 0, 0)
    };
    let got = (le32(&reply.bytes, 0), le16(&reply.bytes, 6), le32(&reply.bytes, 12));
    assert_eq!(got, echo, "the refusal names the request it answers");
    assert_eq!((le16(&reply.bytes, 4), le16(&reply.bytes, 8)), (1, errno));
    assert_eq!(le32(&reply.bytes, 16), 0, "with no body");
    Some(errno)
}

#[test]
fn random_frames_are_served_or_answered_with_a_refusal() {
    let _guard = serial();
    let mut s = 0x4E4E_4554_0000_0001u64;
    let (mut served, mut refused_n) = (0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (FUZZ_BODY_MAX as u64 + 1)) as usize;
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
        let magic = match (r >> 12) % 8 {
            0 => xorshift(&mut s) as u32,
            k => MAGICS[(k as usize - 1) % MAGICS.len()],
        };
        let mut f = frame(magic, op, xorshift(&mut s) as u32, len_field, &body);
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
    let short = frame(MAGIC_NTCP, 3, 7, 0, &[]);
    for f in [&[][..], &short[..1], &short[..HDR_LEN - 1]] {
        assert_eq!(check(f), Some(E_BAD_LEN), "{} bytes", f.len());
    }
    // A bare header is served with an empty body.
    assert_eq!(check(&short), None);
    // A length field one larger than the bytes is refused; one smaller is
    // served with the body it names.
    assert_eq!(check(&frame(MAGIC_NUDP, 4, 9, 5, &[1, 2, 3, 4])), Some(E_BAD_LEN));
    assert_eq!(check(&frame(MAGIC_NUDP, 4, 9, 3, &[1, 2, 3, 4])), None);
    // The length field at its maximum does not wrap into a short frame.
    assert_eq!(check(&frame(MAGIC_NDNS, 2, 11, u32::MAX, &[])), Some(E_BAD_LEN));
    // Every op of every magic, one past the last and the largest, decode as
    // they came: the magic and op are the dispatch's to judge.
    for magic in MAGICS.into_iter().chain([0, u32::MAX]) {
        for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
            assert_eq!(check(&frame(magic, op, u32::from(op), 0, &[])), None, "{magic:#x} op {op}");
        }
    }
    // The largest frame the receive buffer holds is served whole.
    let big = vec![0x5Au8; IPC_BUF_MAX - HDR_LEN];
    let f = frame(MAGIC_NTCP, 5, 13, big.len() as u32, &big);
    assert_eq!(check(&f), None);
    assert_eq!(parse(&f).ok().map(|(_, b)| b.len()), Some(IPC_BUF_MAX - HDR_LEN));
    // A wrong version is answered under the request it carries.
    let mut f = frame(MAGIC_NIP4, 4, 17, 0, &[]);
    f[4] = 2;
    assert_eq!(check(&f), Some(E_BAD_VERSION));
}
