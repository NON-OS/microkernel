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

//! Every frame a client can send the toolkit service, through its real header
//! decode and, when the decode refuses it, the reply the loop answers with.
//! The service is spawned with the desktop and nothing of the system's own
//! calls it, so whoever does is a stranger: each frame must decode to the
//! header it carries or draw one whole reply naming its op and request id
//! (zeros when it is too short to name them).

use crate::protocol::{
    decode, encode, refusal, E_INVAL, E_SHORT, HDR_LEN, IPC_PAYLOAD_MAX, MAGIC,
    TOOLKIT_OP_THEME_GET,
};

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
    let mut f = MAGIC.to_le_bytes().to_vec();
    f.extend_from_slice(&op.to_le_bytes());
    f.extend_from_slice(&[0, 0]);
    f.extend_from_slice(&request_id.to_le_bytes());
    f.extend_from_slice(&payload_len.to_le_bytes());
    f.extend_from_slice(body);
    f
}

/// What must hold for any frame. Returns whether it decoded.
fn check(f: &[u8]) -> bool {
    if let Some(h) = decode(f) {
        assert!(f.len() >= HDR_LEN && le32(f, 0) == MAGIC);
        assert_eq!((h.op, h.request_id, h.payload_len), (le16(f, 4), le32(f, 8), le32(f, 12)));
        return true;
    }
    let (hdr, status) = refusal(f);
    // The loop's refusal arm: encode into the head of its reply buffer, send
    // the header alone.
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    encode(&mut tx[..HDR_LEN], &hdr, status);
    let reply = &tx[..HDR_LEN];
    assert_eq!(le32(reply, 0), MAGIC);
    assert_eq!(le32(reply, 12), 0, "a refusal carries no payload");
    if f.len() < HDR_LEN {
        assert_eq!((le16(reply, 4), le16(reply, 6), le32(reply, 8)), (0, E_SHORT, 0));
    } else {
        assert_eq!((le16(reply, 4), le16(reply, 6), le32(reply, 8)), (le16(f, 4), E_INVAL, le32(f, 8)));
    }
    false
}

#[test]
fn any_frame_decodes_or_draws_a_whole_refusal() {
    let mut s = 0x4E4F_544B_0000_0001u64;
    let mut decoded = 0usize;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let body: Vec<u8> = (0..xorshift(&mut s) % 48).map(|_| xorshift(&mut s) as u8).collect();
        let len_field = match r % 4 {
            0 => xorshift(&mut s) as u32,
            1 => u32::MAX,
            _ => body.len() as u32,
        };
        let op = (xorshift(&mut s) % 7) as u16;
        let mut f = frame(op, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 8) % 8 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
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
    let f = frame(TOOLKIT_OP_THEME_GET, 7, 0, &[]);
    assert!(!check(&f[..HDR_LEN - 1]));
    assert!(check(&f));
    assert!(check(&frame(1, 9, 5, &[1, 2, 3, 4])), "the length field is not the decode's");
    assert!(check(&frame(1, 9, u32::MAX, &[])));
    for op in (0..=TOOLKIT_OP_THEME_GET + 1).chain([u16::MAX]) {
        assert!(check(&frame(op, u32::from(op), 0, &[])), "op {op}");
    }
    let mut bad = frame(1, 11, 0, &[]);
    bad[3] ^= 0x80;
    assert!(!check(&bad));
    assert_eq!(refusal(&bad).0.request_id, 11);
}
