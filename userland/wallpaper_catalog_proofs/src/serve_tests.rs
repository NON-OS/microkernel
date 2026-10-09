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
//! catalog. Each one is answered exactly once, to its sender, with a reply
//! whose header is whole and names the op and index it answers: a caller is
//! blocked in its call until that reply comes, and one never sent costs it
//! the whole timeout. A chunk is the wallpaper's own bytes from the offset
//! asked for, never past its end and never more than a chunk.

use nonos_libc::take_replies;

use crate::catalog::count;
use crate::collection::{reset, wallpaper, TURN};
use crate::protocol::{
    Header, CHUNK_MAX, E_BAD_LEN, E_INVAL, E_NOT_FOUND, E_OK, E_RANGE, HDR_LEN, IPC_PAYLOAD_MAX,
    OP_GET_CHUNK, OP_GET_COUNT, OP_GET_SIZE, OP_GET_SLUG,
};
use crate::server::serve::serve;

const SENDER: u32 = 23;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(op: u16, index: u32, offset: u32, payload_len: u32) -> Vec<u8> {
    let mut f = vec![0u8; HDR_LEN];
    Header { op, status: 0, index, offset, payload_len }.encode(&mut f);
    f
}

/// Serve one frame and return the only reply it drew.
fn answer(f: &[u8]) -> (Header, Vec<u8>) {
    let _ = take_replies();
    serve(SENDER, f);
    let mut replies = take_replies();
    assert_eq!(replies.len(), 1, "one reply for a frame of {} bytes", f.len());
    let (to, reply) = replies.remove(0);
    assert_eq!(to, SENDER);
    assert!(reply.len() >= HDR_LEN && reply.len() <= IPC_PAYLOAD_MAX);
    let Some(hdr) = Header::decode(&reply) else { panic!("a whole header") };
    assert_eq!(hdr.payload_len as usize, reply.len() - HDR_LEN, "the length is the body");
    (hdr, reply[HDR_LEN..].to_vec())
}

/// What must hold for any frame. Returns the reply's status.
fn check(f: &[u8]) -> u16 {
    let (reply, body) = answer(f);
    let Some(req) = Header::decode(f) else {
        assert_eq!((reply.op, reply.index, reply.status), (0, 0, E_BAD_LEN));
        return reply.status;
    };
    // A count names no wallpaper, so its reply carries index zero.
    let index = if req.op == OP_GET_COUNT { 0 } else { req.index };
    assert_eq!((reply.op, reply.index), (req.op, index), "the reply names its request");
    let wallpaper = wallpaper(req.index);
    match (req.op, reply.status) {
        (OP_GET_CHUNK, E_OK) => {
            let bytes = wallpaper.expect("a chunk of a wallpaper that exists");
            let start = req.offset as usize;
            assert!(body.len() <= CHUNK_MAX && start + body.len() <= bytes.len());
            assert_eq!(&body[..], &bytes[start..start + body.len()]);
            assert_eq!(reply.offset, req.offset);
        }
        (OP_GET_CHUNK, E_RANGE) => assert!(req.offset as usize > wallpaper.map_or(0, |b| b.len())),
        (OP_GET_COUNT, E_OK) => assert_eq!(body, count().to_le_bytes()),
        (OP_GET_SIZE | OP_GET_SLUG, E_OK) => assert!(wallpaper.is_some()),
        (OP_GET_CHUNK | OP_GET_SIZE | OP_GET_SLUG, E_NOT_FOUND) => assert!(wallpaper.is_none()),
        (op, E_INVAL) => assert!(![OP_GET_COUNT, OP_GET_SIZE, OP_GET_CHUNK, OP_GET_SLUG].contains(&op)),
        (op, status) => panic!("op {op} answered {status}"),
    }
    reply.status
}

#[test]
fn any_frame_is_answered_once_and_chunks_stay_inside_their_wallpaper() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    let sizes: Vec<u64> = (0..count()).map(|i| wallpaper(i).map_or(0, |b| b.len() as u64)).collect();
    let n = count();
    assert!(n > 0, "the catalog has wallpapers");
    let mut s = 0x5743_4154_0000_0001u64;
    let mut chunks = 0usize;
    for _ in 0..20_000 {
        let r = xorshift(&mut s);
        let op = match r % 8 {
            0 => xorshift(&mut s) as u16,
            1 => (xorshift(&mut s) % 7) as u16,
            2..=5 => OP_GET_CHUNK,
            6 => OP_GET_SIZE,
            _ => OP_GET_SLUG,
        };
        let index = match (r >> 8) % 4 {
            0 => xorshift(&mut s) as u32,
            1 => n + (xorshift(&mut s) % 3) as u32,
            _ => (xorshift(&mut s) % u64::from(n)) as u32,
        };
        let size = sizes.get(index as usize).copied().unwrap_or(0);
        let offset = match (r >> 16) % 5 {
            0 => xorshift(&mut s) as u32,
            1 => (size + (xorshift(&mut s) % 3)).saturating_sub(1) as u32,
            2 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => (xorshift(&mut s) % (size + 1)) as u32,
        };
        let mut f = frame(op, index, offset, xorshift(&mut s) as u32);
        match (r >> 24) & 31 {
            0 | 1 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            2 => f.extend((0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8)),
            _ => {}
        }
        if check(&f) == E_OK && op == OP_GET_CHUNK {
            chunks += 1;
        }
    }
    assert!(chunks > 2_000, "the generator reaches whole chunks: {chunks}");
}

#[test]
fn boundary_frames() {
    let _turn = TURN.lock().unwrap_or_else(|e| e.into_inner());
    reset();
    // Empty, and one byte short of a header.
    for f in [&[][..], &frame(OP_GET_COUNT, 0, 0, 0)[..HDR_LEN - 1]] {
        assert_eq!(check(f), E_BAD_LEN, "{} bytes", f.len());
    }
    // Every op number, one past the last and the largest.
    for op in (0..=OP_GET_SLUG + 1).chain([u16::MAX]) {
        let want = match op {
            OP_GET_COUNT | OP_GET_SIZE | OP_GET_CHUNK | OP_GET_SLUG => E_OK,
            _ => E_INVAL,
        };
        assert_eq!(check(&frame(op, 0, 0, 0)), want, "op {op}");
    }
    // A chunk at the wallpaper's end is empty; one byte past is refused.
    let len = wallpaper(0).map_or(0, |b| b.len()) as u32;
    assert_eq!(check(&frame(OP_GET_CHUNK, 0, len, 0)), E_OK);
    assert_eq!(check(&frame(OP_GET_CHUNK, 0, len + 1, 0)), E_RANGE);
    assert_eq!(check(&frame(OP_GET_CHUNK, 0, u32::MAX, 0)), E_RANGE);
    // The index past the last, and the largest.
    assert_eq!(check(&frame(OP_GET_CHUNK, count(), 0, 0)), E_NOT_FOUND);
    assert_eq!(check(&frame(OP_GET_SLUG, u32::MAX, 0, 0)), E_NOT_FOUND);
    // A length field the service does not read, at its maximum, and a frame
    // longer than its header.
    assert_eq!(check(&frame(OP_GET_COUNT, 0, 0, u32::MAX)), E_OK);
    let mut long = frame(OP_GET_COUNT, 0, 0, 0);
    long.resize(IPC_PAYLOAD_MAX, 0xA5);
    assert_eq!(check(&long), E_OK);
}
