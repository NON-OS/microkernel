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

//! Any frame, from any sender, through `serve` and on to the real handlers and
//! store. Each one is answered exactly once, to its sender, with a reply whose
//! header is whole and names the op and field it answers: a caller is blocked
//! in its call until that reply comes, and one never sent costs it the whole
//! timeout. Only the trusted setter's writes ever succeed, and what a read
//! hands back fits the field's kind.

use nonos_libc::{take_replies, SETTER_PID};
use nonos_policy_proto::{
    decode_field, kind_of, Header, E_ACCES, E_BAD_LEN, E_INVAL, E_OK, HDR_LEN, IPC_PAYLOAD_MAX,
    KIND_STR, OP_GET, OP_SET, OP_STATUS, STR_MAX,
};

use crate::server::serve::serve;

const FUZZ_ROUNDS: usize = 200_000;
const STRANGER_PID: u32 = 77;

/// Field ids the service knows, so the generator reaches the handlers.
const KNOWN_FIELDS: [u32; 8] = [0x0101, 0x0103, 0x0109, 0x0117, 0x0124, 0x0301, 0x0303, 0x0304];

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(op: u16, field: u32, kind: u8, payload_len: u16, body: &[u8]) -> Vec<u8> {
    let mut f = vec![0u8; HDR_LEN];
    Header { op, field, kind, status: 0, payload_len }.encode(&mut f);
    f.extend_from_slice(body);
    f
}

/// Serve one frame and return the only reply it drew.
fn answer(sender: u32, f: &[u8]) -> (Header, Vec<u8>) {
    let _ = take_replies();
    serve(sender, f);
    let mut replies = take_replies();
    assert_eq!(replies.len(), 1, "one reply for a frame of {} bytes", f.len());
    let (to, reply) = replies.remove(0);
    assert_eq!(to, sender, "the reply goes to the sender");
    assert!(reply.len() >= HDR_LEN && reply.len() <= IPC_PAYLOAD_MAX);
    let hdr = Header::decode(&reply).expect("a whole header");
    assert_eq!(usize::from(hdr.payload_len), reply.len() - HDR_LEN, "the length is the body");
    (hdr, reply[HDR_LEN..].to_vec())
}

/// What must hold for any frame, whatever it is. Returns the reply's status.
fn check(sender: u32, f: &[u8]) -> u16 {
    let (reply, payload) = answer(sender, f);
    let Some(req) = Header::decode(f) else {
        assert_eq!((reply.op, reply.field, reply.status), (0, 0, E_BAD_LEN));
        return reply.status;
    };
    assert_eq!((reply.op, reply.field), (req.op, req.field), "the reply names its request");
    let body_fits = f.len() - HDR_LEN >= usize::from(req.payload_len);
    let field = decode_field(req.field);
    if !body_fits || field.is_none() || ![OP_GET, OP_SET].contains(&req.op) {
        assert_eq!(reply.status, E_INVAL);
        return reply.status;
    }
    let field = field.expect("checked above");
    if reply.status != E_OK {
        assert!(payload.is_empty(), "a refusal carries no value");
        return reply.status;
    }
    if req.op == OP_SET {
        assert_eq!(sender, SETTER_PID, "only the trusted setter writes");
        assert!(payload.is_empty());
    } else if kind_of(field) == KIND_STR {
        assert!(payload.len() <= STR_MAX + 1, "a string read of {} bytes", payload.len());
    } else {
        assert_eq!(payload.len(), 1, "a one-byte read");
    }
    reply.status
}

#[test]
fn any_frame_from_any_sender_is_answered_once() {
    let mut s = 0x504F_4C49_0000_0001u64;
    let mut ok = 0usize;
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let op = match r % 8 {
            0 => xorshift(&mut s) as u16,
            1 => OP_STATUS,
            2..=4 => OP_GET,
            _ => OP_SET,
        };
        let field = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u32,
            _ => KNOWN_FIELDS[(xorshift(&mut s) % KNOWN_FIELDS.len() as u64) as usize],
        };
        let body_len = match (r >> 16) % 4 {
            0 => 1,
            1 => (xorshift(&mut s) % (STR_MAX as u64 + 4)) as usize,
            _ => (xorshift(&mut s) % (IPC_PAYLOAD_MAX - HDR_LEN + 1) as u64) as usize,
        };
        let body: Vec<u8> = match (r >> 24) % 3 {
            0 => (0..body_len).map(|_| b'a' + (xorshift(&mut s) % 26) as u8).collect(),
            1 => (0..body_len).map(|_| (xorshift(&mut s) % 3) as u8).collect(),
            _ => (0..body_len).map(|_| xorshift(&mut s) as u8).collect(),
        };
        let len_field = match (r >> 32) % 6 {
            0 => (body_len as u16).wrapping_add(1),
            1 => (body_len as u16).wrapping_sub(1),
            2 => xorshift(&mut s) as u16,
            _ => body_len as u16,
        };
        let mut f = frame(op, field, xorshift(&mut s) as u8, len_field, &body);
        match (r >> 40) & 31 {
            0 | 1 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 2)) as usize),
            2 => f = (0..(xorshift(&mut s) % 40) as usize).map(|_| xorshift(&mut s) as u8).collect(),
            _ => {}
        }
        let sender = if (r >> 48) & 1 == 0 { SETTER_PID } else { STRANGER_PID };
        if check(sender, &f) == E_OK {
            ok += 1;
        }
    }
    assert!(ok > FUZZ_ROUNDS / 20, "the generator reaches the handlers' success paths: {ok}");
}

#[test]
fn boundary_frames() {
    let hostname = 0x0301;
    // Empty, and one byte short of a header.
    for f in [&[][..], &frame(OP_GET, hostname, 0, 0, &[])[..HDR_LEN - 1]] {
        let (reply, _) = answer(SETTER_PID, f);
        assert_eq!(reply.status, E_BAD_LEN, "{} bytes", f.len());
    }
    // A bare header is a read.
    let (reply, _) = answer(STRANGER_PID, &frame(OP_GET, hostname, 0, 0, &[]));
    assert_eq!(reply.status, E_OK);
    // A length field one larger than the bytes, and one smaller.
    let (reply, _) = answer(SETTER_PID, &frame(OP_SET, hostname, 0, 5, b"node"));
    assert_eq!(reply.status, E_INVAL);
    let (reply, _) = answer(SETTER_PID, &frame(OP_SET, hostname, 0, 3, b"node"));
    assert_eq!(reply.status, E_OK, "the body is the bytes the length names");
    // The length field at its maximum.
    let (reply, _) = answer(SETTER_PID, &frame(OP_SET, hostname, 0, u16::MAX, b"node"));
    assert_eq!(reply.status, E_INVAL);
    // Every op number the service could be asked, and the largest.
    for op in [0, OP_GET, OP_SET, OP_STATUS, OP_STATUS + 1, u16::MAX] {
        let (reply, _) = answer(STRANGER_PID, &frame(op, hostname, 0, 0, &[]));
        let want = match op {
            OP_GET => E_OK,
            OP_SET => E_ACCES,
            _ => E_INVAL,
        };
        assert_eq!((reply.op, reply.status), (op, want), "op {op}");
    }
    // The largest frame the receive buffer holds: a string far past its limit.
    let body = vec![b'a'; IPC_PAYLOAD_MAX - HDR_LEN];
    let (reply, _) = answer(SETTER_PID, &frame(OP_SET, hostname, 0, body.len() as u16, &body));
    assert_eq!(reply.status, E_BAD_LEN);
    // A field the service does not know.
    let (reply, _) = answer(STRANGER_PID, &frame(OP_GET, 0xFFFF_FFFF, 0, 0, &[]));
    assert_eq!((reply.field, reply.status), (0xFFFF_FFFF, E_INVAL));
}
