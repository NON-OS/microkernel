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

//! Any bytes a client can put in the driver's inbox, through the first step of
//! its serve loop, the shared link decode and dispatch (`netif::serve`, here
//! against the driver's own down link), and, for every frame that step does
//! not take, the refusal the loop now sends when the control family does not
//! take it either. Before, such a frame got no reply at all, so its caller
//! waited out its whole timeout and its entry stayed in the kernel's reply
//! queue for the driver.
//!
//! The control family's own gate (a 10 byte header under its magic) needs the
//! radio and is not included; what is proven is that each frame the link step
//! leaves is answered by `refuse` with one whole reply naming what it carried.

use nonos_wifi_core::netif::{serve, wire, MAX_RESPONSE};

use crate::link::DeadLink;
use crate::serve_refuse::{refuse, STATUS_REFUSED};

const FUZZ_ROUNDS: usize = 200_000;
/// The receive buffer the loop hands the step a slice of.
const RX_LEN: usize = wire::HDR_LEN + wire::MAX_FRAME;
/// The control family's magic, "WIFI", which its gate looks for.
const WIFI_MAGIC: u32 = 0x5749_4649;
/// The last op the link family knows.
const LAST_OP: u16 = wire::OP_RX_PACKET;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn frame(magic: u32, op: u16, request_id: u32, payload_len: u32, body: &[u8]) -> Vec<u8> {
    let mut f = Vec::with_capacity(wire::HDR_LEN + body.len());
    f.extend_from_slice(&magic.to_le_bytes());
    f.extend_from_slice(&wire::VERSION.to_le_bytes());
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

/// The refusal `refuse` writes for `req`, checked whole.
fn check_refusal(req: &[u8]) {
    let mut out = [0xEEu8; MAX_RESPONSE];
    let n = refuse(req, &mut out);
    assert_eq!(n, wire::HDR_LEN + 4, "a header and a status for {} bytes", req.len());
    let reply = &out[..n];
    assert_eq!((le32(reply, 0), le16(reply, 4)), (wire::MAGIC_NNET, wire::VERSION));
    let echo = if req.len() >= wire::HDR_LEN { (le16(req, 6), le32(req, 12)) } else { (0, 0) };
    assert_eq!((le16(reply, 6), le32(reply, 12)), echo, "the refusal names the request it answers");
    assert_eq!((le16(reply, 8), le16(reply, 10), le32(reply, 16)), (0, 0, 4));
    assert_eq!(le32(reply, wire::HDR_LEN) as i32, STATUS_REFUSED);
    assert!(out[n..].iter().all(|&b| b == 0xEE), "nothing written past the reply");
}

/// The loop's first step for `req`, then the refusal for what it leaves.
/// Returns whether the link step took it.
fn check(req: &[u8]) -> bool {
    let mut out = [0u8; MAX_RESPONSE];
    match serve(req, &mut DeadLink, &mut out) {
        Some(n) => {
            assert!((wire::HDR_LEN..=MAX_RESPONSE).contains(&n), "a reply of {n} bytes");
            assert_eq!(le32(&out, 0), wire::MAGIC_NNET);
            assert_eq!((le16(&out, 6), le32(&out, 12)), (le16(req, 6), le32(req, 12)));
            true
        }
        None => {
            check_refusal(req);
            false
        }
    }
}

#[test]
fn random_frames_are_served_or_answered_with_a_refusal() {
    let mut s = 0x5254_4C38_0000_0001u64;
    let (mut served, mut left, mut left_unclaimed) = (0usize, 0usize, 0usize);
    for _ in 0..FUZZ_ROUNDS {
        let r = xorshift(&mut s);
        let body_len = (xorshift(&mut s) % (wire::MAX_FRAME as u64 + 1)) as usize;
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
            _ => (xorshift(&mut s) % (u64::from(LAST_OP) + 3)) as u16,
        };
        let magic = match (r >> 12) % 8 {
            0 => WIFI_MAGIC,
            1 => xorshift(&mut s) as u32,
            _ => wire::MAGIC_NNET,
        };
        let mut f = frame(magic, op, xorshift(&mut s) as u32, len_field, &body);
        match (r >> 16) % 16 {
            0 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            1 => f[4] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.truncate((xorshift(&mut s) % (wire::HDR_LEN as u64 + 1)) as usize),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        f.truncate(RX_LEN);
        if check(&f) {
            served += 1;
        } else {
            left += 1;
            // Not a control request either: the frames that went unanswered.
            if f.len() < 10 || le32(&f, 0) != WIFI_MAGIC {
                left_unclaimed += 1;
            }
        }
    }
    assert!(served > FUZZ_ROUNDS / 10, "the generator reaches the link step: {served}");
    assert!(left > FUZZ_ROUNDS / 4, "and what it leaves: {left}");
    assert!(left_unclaimed > FUZZ_ROUNDS / 10, "including what no family takes: {left_unclaimed}");
}

#[test]
fn boundary_frames() {
    // Empty, one byte, one short of a header, and a control header one byte
    // short of its 10: none is taken, each is refused under zeros.
    let bare = frame(wire::MAGIC_NNET, wire::OP_LINK_STATUS, 7, 0, &[]);
    for cut in [0, 1, wire::HDR_LEN - 1] {
        assert!(!check(&bare[..cut]), "{cut} bytes");
    }
    assert!(!check(&WIFI_MAGIC.to_le_bytes().repeat(3)[..9]));
    // A bare header of each link op is served; an op past them, op 0 and the
    // largest are left, then refused naming the op and request id.
    for op in [wire::OP_LINK_STATUS, wire::OP_MAC_ADDRESS, wire::OP_TX_PACKET, wire::OP_RX_PACKET] {
        assert!(check(&frame(wire::MAGIC_NNET, op, u32::from(op), 0, &[])), "op {op}");
    }
    for op in [0, 1, LAST_OP + 1, u16::MAX] {
        assert!(!check(&frame(wire::MAGIC_NNET, op, 0xA0 + u32::from(op), 0, &[])), "op {op}");
    }
    // Length fields one over, one under and at their maximum.
    assert!(!check(&frame(wire::MAGIC_NNET, wire::OP_TX_PACKET, 9, 5, &[1, 2, 3, 4])));
    assert!(check(&frame(wire::MAGIC_NNET, wire::OP_TX_PACKET, 9, 3, &[1, 2, 3, 4])));
    assert!(!check(&frame(wire::MAGIC_NNET, wire::OP_TX_PACKET, 11, u32::MAX, &[])));
    // The largest frame the receive buffer holds.
    let big = vec![0x5Au8; wire::MAX_FRAME];
    assert!(check(&frame(wire::MAGIC_NNET, wire::OP_TX_PACKET, 13, big.len() as u32, &big)));
    // A wrong magic and a wrong version are refused naming what they carried.
    let mut f = frame(wire::MAGIC_NNET, wire::OP_LINK_STATUS, 15, 0, &[]);
    f[0] ^= 0xFF;
    assert!(!check(&f));
    let mut f = frame(wire::MAGIC_NNET, wire::OP_LINK_STATUS, 17, 0, &[]);
    f[4] = 2;
    assert!(!check(&f));
    // A response buffer too short for a header and a status gets nothing.
    let mut short = [0xEEu8; wire::HDR_LEN + 3];
    assert_eq!(refuse(&bare, &mut short), 0);
    assert!(short.iter().all(|&b| b == 0xEE));
    let mut exact = [0u8; wire::HDR_LEN + 4];
    assert_eq!(refuse(&bare, &mut exact), wire::HDR_LEN + 4);
}
