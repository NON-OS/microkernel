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

//! Every frame a client can send audio.server, through the real header
//! decode and the reply a refused frame gets. The server answers a frame that
//! does not decode, and every frame while there is no sink to play on, with
//! E_INVAL under `refused`; these pin that the decode takes any bytes without
//! a panic, and that the refusal is a whole reply naming the op and request id
//! the frame carried, or zeros when it is too short to carry them.

use crate::server::proto::{decode, encode_reply, refused, Request};
use nonos_audio_proto::{E_INVAL, HDR_LEN, MAGIC, OP_RESUME, STATUS_LEN, VERSION};

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

fn frame(op: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::new();
    f.extend_from_slice(&MAGIC.to_le_bytes());
    f.extend_from_slice(&VERSION.to_le_bytes());
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0, 0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// The refusal for `f`, as the server builds and sends it.
fn refusal(f: &[u8]) -> Vec<u8> {
    let mut tx = vec![0u8; 28];
    let n = encode_reply(&refused(f), E_INVAL, &mut tx);
    tx.truncate(n);
    tx
}

/// What must hold for any frame.
fn check(f: &[u8]) -> bool {
    let decoded: Option<Request> = decode(f);
    let whole = f.len() >= HDR_LEN && le32(f, 0) == MAGIC && le16(f, 4) == VERSION;
    assert_eq!(decoded.is_some(), whole, "decode takes exactly the frames with a whole header");
    if let Some(req) = &decoded {
        assert_eq!(
            (req.op, req.request_id, req.payload_len),
            (le16(f, 6), le32(f, 12), le32(f, 16))
        );
    }
    let r = refusal(f);
    assert_eq!(r.len(), HDR_LEN + STATUS_LEN, "a whole reply for a frame of {} bytes", f.len());
    assert_eq!((le32(&r, 0), le16(&r, 4), le32(&r, 16)), (MAGIC, VERSION, STATUS_LEN as u32));
    let (op, id) = if f.len() >= HDR_LEN { (le16(f, 6), le32(f, 12)) } else { (0, 0) };
    assert_eq!((le16(&r, 6), le32(&r, 12)), (op, id), "the refusal names its request");
    assert_eq!(le32(&r, HDR_LEN) as i32, E_INVAL);
    decoded.is_some()
}

#[test]
fn any_frame_decodes_or_draws_a_whole_refusal() {
    let mut s = 0x4E41_5544_0000_0001u64;
    let mut decoded = 0usize;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let body: Vec<u8> = (0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 4 {
            0 => xorshift(&mut s) as u32,
            1 => u32::MAX,
            _ => body.len() as u32,
        };
        let op = (xorshift(&mut s) % (u64::from(OP_RESUME) + 2)) as u16;
        let mut f = frame(op, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 8) % 8 {
            0 => f[(xorshift(&mut s) % 6) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            2 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
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
    let f = frame(1, 7, 0, &[]);
    assert!(!check(&f[..HDR_LEN - 1]));
    assert!(check(&f));
    // The length field is the dispatch's to bound, so any value decodes.
    assert!(check(&frame(1, 9, 5, &[1, 2, 3, 4])));
    assert!(check(&frame(1, 9, 3, &[1, 2, 3, 4])));
    assert!(check(&frame(1, 9, u32::MAX, &[])));
    for op in (0..=OP_RESUME + 1).chain([u16::MAX]) {
        assert!(check(&frame(op, u32::from(op), 0, &[])), "op {op}");
    }
    let mut bad = frame(2, 11, 0, &[]);
    bad[0] ^= 1;
    assert!(!check(&bad));
    // A reply buffer too small for a header and status gets no reply at all,
    // which the server's 28-byte buffer never is.
    assert_eq!(encode_reply(&refused(&f), E_INVAL, &mut [0u8; HDR_LEN + STATUS_LEN - 1]), 0);
}
