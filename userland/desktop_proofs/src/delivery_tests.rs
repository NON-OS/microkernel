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

//! Every app built on app_skeleton reads its inbox with parse_delivery, and
//! any process can put bytes there. The decode takes any frame without a
//! panic and yields an event only from a frame long enough to hold one, with
//! the delivery magic and a known kind, with each field read from its own
//! offset. What it yields is then honoured only from the input router; that
//! check is a kernel lookup and is not run here.

use super::dispatch::{parse_delivery, DELIVERY_LEN, NINP_HDR_LEN};
use crate::input::{InputEvent, InputKind};
use crate::wire::NINP_MAGIC;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn le32(b: &[u8], off: usize) -> u32 {
    u32::from_le_bytes([b[off], b[off + 1], b[off + 2], b[off + 3]])
}

fn delivery(kind: u16, code: u32, x: i32, y: i32) -> Vec<u8> {
    let mut f = NINP_MAGIC.to_le_bytes().to_vec();
    f.extend_from_slice(&[1, 0, 0, 0]);
    f.extend_from_slice(&kind.to_le_bytes());
    f.extend_from_slice(&0u16.to_le_bytes());
    f.extend_from_slice(&code.to_le_bytes());
    f.extend_from_slice(&x.to_le_bytes());
    f.extend_from_slice(&y.to_le_bytes());
    f.extend_from_slice(&[0; 16]);
    f
}

#[test]
fn any_frame_yields_an_event_only_when_whole() {
    let mut s = 0x4E49_4E50_0000_0001u64;
    let mut events = 0usize;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let (kind, code) = ((xorshift(&mut s) % 10) as u16, xorshift(&mut s) as u32);
        let mut f = delivery(kind, code, xorshift(&mut s) as i32, xorshift(&mut s) as i32);
        match r % 8 {
            0 => f.truncate((xorshift(&mut s) % DELIVERY_LEN as u64) as usize),
            1 => f[(xorshift(&mut s) % 4) as usize] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.extend((0..xorshift(&mut s) % 16).map(|_| xorshift(&mut s) as u8)),
            3 => f.iter_mut().for_each(|b| *b = xorshift(&mut s) as u8),
            _ => {}
        }
        let Some(ev) = parse_delivery(&f) else { continue };
        events += 1;
        assert!(f.len() >= DELIVERY_LEN && le32(&f, 0) == NINP_MAGIC);
        let p = &f[NINP_HDR_LEN..];
        assert!(u16::from_le_bytes([p[0], p[1]]) <= 7, "a known kind");
        assert_eq!((ev.code, ev.x, ev.y), (le32(p, 4), le32(p, 8) as i32, le32(p, 12) as i32));
    }
    assert!(events > 50_000, "the generator reaches whole events: {events}");
}

#[test]
fn boundary_frames() {
    assert!(parse_delivery(&[]).is_none());
    let f = delivery(0, 0x41, 1, 2);
    assert!(parse_delivery(&f[..DELIVERY_LEN - 1]).is_none(), "one byte short");
    assert!(parse_delivery(&f).is_some());
    for kind in 0..=8u16 {
        let ev = parse_delivery(&delivery(kind, 0, 0, 0));
        assert_eq!(ev.is_some(), kind <= 7, "kind {kind}");
    }
    let ev = parse_delivery(&delivery(4, u32::MAX, i32::MIN, i32::MAX));
    assert!(matches!(ev.map(|e| e.kind), Some(InputKind::Wheel)));
    let mut bad = f.clone();
    bad[0] ^= 1;
    assert!(parse_delivery(&bad).is_none(), "the wrong magic");
}

/// The event decode on its own, as the Linux guest's wayland input and the
/// installer call it after their own header checks: it reads only a whole
/// 32-byte event, whatever it is handed.
#[test]
fn the_event_decode_reads_only_a_whole_event() {
    for n in 0..48 {
        let p = vec![0u8; n];
        assert_eq!(InputEvent::from_delivery(&p).is_some(), n >= 32, "{n} bytes");
    }
}
