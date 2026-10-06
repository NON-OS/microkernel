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

//! Every frame a client can put in the input router's inbox, through the
//! real header decode. Any process holding the endpoint can send any bytes,
//! so the decode must take each one without a panic and either serve it or
//! refuse it with a status and the request the refusal answers. The loop
//! replies to a refusal with exactly what these build, so a refused caller
//! hears back at once instead of waiting out its timeout.

use crate::router_protocol::{
    parse, read_u32, response_header, write_status, Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION,
    HDR_LEN, IPC_PAYLOAD_MAX, MAGIC, OP_GRAB_RELEASE, OP_HEALTHCHECK, OP_SUBSCRIBE, STATUS_LEN,
    SUBSCRIBE_REQ_LEN, VERSION,
};

const FUZZ_ROUNDS: usize = 250_000;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(op: u16, flags: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(HDR_LEN + body.len());
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&flags.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
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

/// The reply the loop sends for a refusal, built by the encoders it uses.
fn refusal_reply(req: &Request, code: i32) -> Vec<u8> {
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    response_header(&mut tx, req, STATUS_LEN as u32);
    write_status(&mut tx, code);
    tx.truncate(HDR_LEN + STATUS_LEN);
    tx
}

/// What must hold for any input, whatever it is.
fn check(buf: &[u8]) {
    match parse(buf) {
        Ok((req, body)) => {
            assert!(buf.len() >= HDR_LEN);
            assert_eq!(le32(buf, 0), MAGIC);
            assert_eq!(le16(buf, 4), VERSION);
            assert_eq!(req.op, le16(buf, 6));
            assert_eq!(req.flags, le16(buf, 8));
            assert_eq!(req.request_id, le32(buf, 12));
            assert_eq!(body.len() as u64, u64::from(le32(buf, 16)));
            assert_eq!(body, &buf[HDR_LEN..], "the body is the rest of the frame");
        }
        Err((code, req)) => {
            assert!([E_BAD_LEN, E_BAD_MAGIC, E_BAD_VERSION].contains(&code), "status {code}");
            if buf.len() < HDR_LEN {
                assert_eq!(code, E_BAD_LEN);
                assert_eq!((req.op, req.flags, req.request_id), (0, 0, 0));
            } else {
                assert_eq!(
                    (req.op, req.flags, req.request_id),
                    (le16(buf, 6), le16(buf, 8), le32(buf, 12))
                );
            }
            let reply = refusal_reply(&req, code);
            assert_eq!(reply.len(), HDR_LEN + STATUS_LEN);
            assert_eq!(le32(&reply, 0), MAGIC);
            assert_eq!(le16(&reply, 4), VERSION);
            assert_eq!(le16(&reply, 6), req.op);
            assert_eq!(le32(&reply, 12), req.request_id);
            assert_eq!(le32(&reply, 16), STATUS_LEN as u32);
            assert_eq!(le32(&reply, HDR_LEN) as i32, code);
        }
    }
}

#[test]
fn random_frames_are_served_or_refused_with_a_reply() {
    let mut s = 0x4E49_5253_0000_0001u64;
    let (mut served, mut refused) = (0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (IPC_PAYLOAD_MAX as u64 + 8)) as usize;
        let body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).wrapping_sub(1),
            2 => xorshift(&mut s) as u32,
            3 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let op = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u16,
            _ => (xorshift(&mut s) % 0x10) as u16,
        };
        let mut f = frame(op, xorshift(&mut s) as u16, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            3 => {
                for b in f.iter_mut() {
                    *b = xorshift(&mut s) as u8;
                }
            }
            _ => {}
        }
        if parse(&f).is_ok() {
            served += 1;
        } else {
            refused += 1;
        }
        check(&f);
    }
    assert!(served > FUZZ_ROUNDS / 4, "the generator reaches the dispatch: {served}");
    assert!(refused > FUZZ_ROUNDS / 4, "the generator reaches the refusals: {refused}");
}

#[test]
fn boundary_frames() {
    // Empty, and one byte short of a header: refused under zeros.
    check(&[]);
    assert_eq!(parse(&[]).err().map(|(c, _)| c), Some(E_BAD_LEN));
    let short = frame(OP_HEALTHCHECK, 0, 7, 0, &[]);
    check(&short[..HDR_LEN - 1]);
    assert_eq!(parse(&short[..HDR_LEN - 1]).err().map(|(c, _)| c), Some(E_BAD_LEN));
    // A bare header with an empty body is served.
    assert!(parse(&short).is_ok());
    // A length field larger, and smaller, than the bytes that came.
    for (len_field, body) in [(5u32, &[1u8, 2, 3, 4][..]), (3, &[1, 2, 3, 4][..])] {
        let f = frame(OP_HEALTHCHECK, 0, 9, len_field, body);
        check(&f);
        let (code, req) = parse(&f).err().expect("a length that disagrees is refused");
        assert_eq!((code, req.request_id), (E_BAD_LEN, 9));
    }
    // The length field at its maximum does not wrap into a short frame.
    let f = frame(OP_HEALTHCHECK, 0, 11, u32::MAX, &[]);
    check(&f);
    assert_eq!(parse(&f).err().map(|(c, _)| c), Some(E_BAD_LEN));
    // Every op the input router knows, one past the last, and the largest.
    for op in (0..=OP_GRAB_RELEASE + 1).chain([u16::MAX]) {
        let f = frame(op, 3, u32::from(op) + 100, 0, &[]);
        check(&f);
        let (req, body) = parse(&f).ok().expect("a well formed frame is served");
        assert_eq!((req.op, req.request_id, body.len()), (op, u32::from(op) + 100, 0));
    }
    // The largest body the receive buffer holds is served whole.
    let big = vec![0xA5u8; IPC_PAYLOAD_MAX];
    let f = frame(OP_SUBSCRIBE, 0, 13, IPC_PAYLOAD_MAX as u32, &big);
    check(&f);
    assert_eq!(parse(&f).ok().map(|(_, b)| b.len()), Some(IPC_PAYLOAD_MAX));
    // Wrong magic and wrong version echo the request they refuse.
    let mut f = frame(OP_HEALTHCHECK, 0, 15, 0, &[]);
    f[0] ^= 0xFF;
    assert_eq!(parse(&f).err().map(|(c, r)| (c, r.request_id)), Some((E_BAD_MAGIC, 15)));
    let mut f = frame(OP_HEALTHCHECK, 0, 17, 0, &[]);
    f[4] = 2;
    assert_eq!(parse(&f).err().map(|(c, r)| (c, r.request_id)), Some((E_BAD_VERSION, 17)));
}

#[test]
fn the_body_field_reader_stays_inside_the_body() {
    let mut s = 0x5245_4144_0000_0001u64;
    for _ in 0..200_000 {
        let len = (xorshift(&mut s) % (SUBSCRIBE_REQ_LEN as u64 * 2)) as usize;
        let body: Vec<u8> = (0..len).map(|_| xorshift(&mut s) as u8).collect();
        let off = match xorshift(&mut s) % 4 {
            0 => usize::MAX - (xorshift(&mut s) % 4) as usize,
            _ => (xorshift(&mut s) % (len as u64 + 6)) as usize,
        };
        match read_u32(&body, off) {
            Some(v) => {
                assert!(off + 4 <= len);
                assert_eq!(v, le32(&body, off));
            }
            None => assert!(off.checked_add(4).is_none_or(|end| end > len)),
        }
    }
}
