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

//! Any frame through `route`, the function the clipboard's loop hands every
//! received frame to and whose returned length it sends back. The loop sends
//! nothing when route returns zero, so route must return a whole reply for
//! every input: a header naming the op, flags and request id it answers (zeros
//! for a frame too short to name them), a length field that is the rest of the
//! reply, and a status. Whatever the requests, the history stays within its
//! depth and byte bounds, and a paste hands back what was copied.

use crate::protocol::{
    Request, E_BAD_LEN, E_BAD_MAGIC, E_BAD_OP, E_BAD_VERSION, E_INVAL, E_RANGE, HDR_LEN,
    IPC_PAYLOAD_MAX, MAGIC, MAX_DEPTH, MAX_ENTRY_BYTES, MAX_IDLE_TIMEOUT_MS, MAX_TOTAL_BYTES,
    MIN_IDLE_TIMEOUT_MS, OP_CLEAR, OP_COPY, OP_HEALTHCHECK, OP_HISTORY_GET, OP_HISTORY_LIST,
    OP_PASTE, OP_SET_IDLE_TIMEOUT, STATUS_LEN, VERSION,
};
use crate::server::handlers::route;
use crate::state::Clipboard;

const FUZZ_ROUNDS: usize = 200_000;
const NOW: u64 = 1_000_000;

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

fn fresh() -> Clipboard {
    Clipboard::new(MAX_DEPTH, MAX_TOTAL_BYTES, 0, NOW)
}

/// Route one frame as the loop does and return the reply it would send.
fn answer(clip: &mut Clipboard, input: &[u8]) -> Vec<u8> {
    let mut out = vec![0u8; IPC_PAYLOAD_MAX];
    let n = route(clip, &input[..input.len().min(IPC_PAYLOAD_MAX)], &mut out, NOW);
    assert!(n >= HDR_LEN + STATUS_LEN && n <= out.len(), "a whole reply, {n} bytes");
    out.truncate(n);
    let echo = if input.len() >= HDR_LEN {
        Request { op: le16(input, 6), flags: le16(input, 8), request_id: le32(input, 12) }
    } else {
        Request { op: 0, flags: 0, request_id: 0 }
    };
    assert_eq!((le32(&out, 0), le16(&out, 4)), (MAGIC, VERSION));
    assert_eq!((le16(&out, 6), le16(&out, 8), le32(&out, 12)), (echo.op, echo.flags, echo.request_id));
    assert_eq!(le32(&out, 16) as usize, n - HDR_LEN, "the length field is the rest of the reply");
    out
}

fn status(reply: &[u8]) -> i32 {
    le32(reply, HDR_LEN) as i32
}

/// The history's depth and total bytes, read back through OP_HISTORY_LIST.
fn history(clip: &mut Clipboard) -> (usize, usize) {
    let r = answer(clip, &frame(OP_HISTORY_LIST, 0, 1, 0, &[]));
    assert_eq!(status(&r), 0);
    let count = le32(&r, HDR_LEN + STATUS_LEN) as usize;
    assert_eq!(r.len(), HDR_LEN + STATUS_LEN + 4 + count * 8);
    let total = (0..count).map(|i| le32(&r, HDR_LEN + STATUS_LEN + 8 + i * 8) as usize).sum();
    (count, total)
}

#[test]
fn any_frame_draws_a_whole_reply_and_the_history_stays_bounded() {
    let mut s = 0x4342_4930_0000_0001u64;
    let mut clip = fresh();
    let mut copied = 0usize;
    for round in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let op = match r % 8 {
            0 => xorshift(&mut s) as u16,
            1 => (xorshift(&mut s) % 9) as u16,
            2 | 3 => OP_COPY,
            4 => OP_PASTE,
            5 => OP_HISTORY_GET,
            6 => OP_SET_IDLE_TIMEOUT,
            _ => OP_HISTORY_LIST,
        };
        let body_len = match (r >> 8) % 64 {
            0 => MAX_ENTRY_BYTES + 4 - (xorshift(&mut s) % 3) as usize + 1,
            1..=8 => (xorshift(&mut s) % 12) as usize,
            _ => (xorshift(&mut s) % 300) as usize,
        };
        let mut body: Vec<u8> = (0..body_len).map(|_| xorshift(&mut s) as u8).collect();
        if body.len() >= 4 && (r >> 16) & 1 == 0 {
            body[..4].copy_from_slice(&((xorshift(&mut s) % 3) as u32).to_le_bytes());
        }
        let len_field = match (r >> 20) % 8 {
            0 => body_len as u32 + 1,
            1 => (body_len as u32).saturating_sub(1),
            2 => u32::MAX - (xorshift(&mut s) % 4) as u32,
            _ => body_len as u32,
        };
        let mut f = frame(op, xorshift(&mut s) as u16, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 24) % 16 {
            0 => f[(xorshift(&mut s) % 6) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            _ => {}
        }
        let reply = answer(&mut clip, &f);
        if op == OP_COPY && status(&reply) == 0 && f.len() >= HDR_LEN + 4 {
            copied += 1;
        }
        if round & 63 == 0 {
            let (depth, total) = history(&mut clip);
            assert!(depth <= MAX_DEPTH && total <= MAX_TOTAL_BYTES, "{depth} entries, {total} bytes");
        }
    }
    assert!(copied > FUZZ_ROUNDS / 20, "the generator reaches the copy path: {copied}");
}

#[test]
fn boundary_frames() {
    let mut clip = fresh();
    // Empty, and one byte short of a header: refused under zeros.
    let short = frame(OP_HEALTHCHECK, 0, 7, 0, &[]);
    for f in [&[][..], &short[..HDR_LEN - 1]] {
        assert_eq!(status(&answer(&mut clip, f)), E_BAD_LEN, "{} bytes", f.len());
    }
    assert_eq!(status(&answer(&mut clip, &short)), 0);
    // A length field one larger than the bytes, one smaller, and its maximum.
    let one_over = frame(OP_COPY, 0, 9, 9, &[1, 0, 0, 0, b'a', b'b', b'c', b'd']);
    assert_eq!(status(&answer(&mut clip, &one_over)), E_BAD_LEN);
    let one_under = frame(OP_COPY, 0, 9, 7, &[1, 0, 0, 0, b'a', b'b', b'c', b'd']);
    assert_eq!(status(&answer(&mut clip, &one_under)), 0);
    let paste = answer(&mut clip, &frame(OP_PASTE, 0, 10, 4, &[1, 0, 0, 0]));
    assert_eq!(&paste[HDR_LEN + STATUS_LEN + 4..], b"abc", "the body is what the length names");
    assert_eq!(status(&answer(&mut clip, &frame(OP_COPY, 0, 11, u32::MAX, &[0; 8]))), E_BAD_LEN);
    // Wrong magic and version.
    let mut f = frame(OP_HEALTHCHECK, 0, 12, 0, &[]);
    f[0] ^= 1;
    assert_eq!(status(&answer(&mut clip, &f)), E_BAD_MAGIC);
    let mut f = frame(OP_HEALTHCHECK, 0, 13, 0, &[]);
    f[4] = 9;
    assert_eq!(status(&answer(&mut clip, &f)), E_BAD_VERSION);
    // Every op with an empty body, one past the last, and the largest.
    for op in (0..=OP_SET_IDLE_TIMEOUT + 1).chain([u16::MAX]) {
        let want = match op {
            OP_HEALTHCHECK | OP_HISTORY_LIST | OP_CLEAR => 0,
            OP_COPY | OP_PASTE | OP_HISTORY_GET | OP_SET_IDLE_TIMEOUT => E_INVAL,
            _ => E_BAD_OP,
        };
        assert_eq!(status(&answer(&mut clip, &frame(op, 0, 14, 0, &[]))), want, "op {op}");
    }
    // The largest entry fits the receive buffer and pastes back whole; one
    // byte more is refused.
    let mut body = vec![2u8, 0, 0, 0];
    body.extend(std::iter::repeat_n(0xA5u8, MAX_ENTRY_BYTES));
    assert_eq!(status(&answer(&mut clip, &frame(OP_COPY, 0, 15, body.len() as u32, &body))), 0);
    let p = answer(&mut clip, &frame(OP_PASTE, 0, 16, 4, &[2, 0, 0, 0]));
    assert_eq!(p.len(), HDR_LEN + STATUS_LEN + 4 + MAX_ENTRY_BYTES);
    assert_eq!(p.len(), IPC_PAYLOAD_MAX - 4, "the largest paste fits the reply buffer");
    let g = answer(&mut clip, &frame(OP_HISTORY_GET, 0, 17, 4, &[0, 0, 0, 0]));
    assert_eq!(g.len(), HDR_LEN + STATUS_LEN + 8 + MAX_ENTRY_BYTES);
    body.push(0);
    assert_eq!(status(&answer(&mut clip, &frame(OP_COPY, 0, 18, body.len() as u32, &body))), E_RANGE);
    // History indexes past the end, and idle timeouts outside their bounds.
    let past = (MAX_DEPTH as u32).to_le_bytes();
    assert_eq!(status(&answer(&mut clip, &frame(OP_HISTORY_GET, 0, 19, 4, &past))), E_RANGE);
    for t in [1, MIN_IDLE_TIMEOUT_MS - 1, MAX_IDLE_TIMEOUT_MS + 1, u64::MAX] {
        let f = frame(OP_SET_IDLE_TIMEOUT, 0, 20, 8, &t.to_le_bytes());
        assert_eq!(status(&answer(&mut clip, &f)), E_RANGE, "timeout {t}");
    }
}

#[test]
fn the_largest_copies_evict_the_oldest_to_stay_within_the_byte_bound() {
    let mut clip = fresh();
    let per = MAX_TOTAL_BYTES / MAX_ENTRY_BYTES;
    for i in 0..=per as u32 {
        let mut body = (i + 100).to_le_bytes().to_vec();
        body.extend(std::iter::repeat_n(i as u8, MAX_ENTRY_BYTES));
        let f = frame(OP_COPY, 0, i, body.len() as u32, &body);
        assert_eq!(status(&answer(&mut clip, &f)), 0);
        let (depth, total) = history(&mut clip);
        assert!(total <= MAX_TOTAL_BYTES, "{total} bytes in {depth} entries");
    }
    assert_eq!(history(&mut clip), (per, per * MAX_ENTRY_BYTES), "the oldest went first");
    let gone = answer(&mut clip, &frame(OP_PASTE, 0, 1, 4, &100u32.to_le_bytes()));
    assert_eq!(gone.len(), HDR_LEN + STATUS_LEN, "the first copy was evicted");
}
