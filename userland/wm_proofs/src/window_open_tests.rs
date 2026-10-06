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

//! The OP_WINDOW_OPEN body any client can send, through the real decode. It
//! takes exactly 24 bytes, refuses a zero id, size or an unknown kind, and
//! whatever it accepts lands on the display whatever the numbers were.

use super::window_open_decode::decode;
use crate::protocol::WINDOW_OPEN_REQ_LEN;

fn xorshift(s: &mut u64) -> u64 {
    let mut x = *s;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *s = x;
    x
}

fn body(id: u32, kind: u32, x: u32, y: u32, w: u32, h: u32) -> Vec<u8> {
    [id, kind, x, y, w, h].iter().flat_map(|v| v.to_le_bytes()).collect()
}

#[test]
fn random_bodies_decode_onto_the_display_or_not_at_all() {
    let mut s = 0x4F50_454E_0000_0001u64;
    let mut accepted = 0usize;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let pick = |s: &mut u64| match xorshift(s) % 4 {
            0 => 0,
            1 => u32::MAX - (xorshift(s) % 3) as u32,
            2 => (xorshift(s) % 4096) as u32,
            _ => xorshift(s) as u32,
        };
        let kind = (xorshift(&mut s) % 6) as u32;
        let mut b = body(pick(&mut s), kind, pick(&mut s), pick(&mut s), pick(&mut s), pick(&mut s));
        match r % 8 {
            0 => b.truncate((xorshift(&mut s) % WINDOW_OPEN_REQ_LEN as u64) as usize),
            1 => b.push(xorshift(&mut s) as u8),
            _ => {}
        }
        let (dw, dh) = ((xorshift(&mut s) % 5000) as u32, (xorshift(&mut s) % 5000) as u32);
        let Some((id, _, rect)) = decode(&b, dw, dh) else { continue };
        accepted += 1;
        assert_eq!(b.len(), WINDOW_OPEN_REQ_LEN);
        assert_ne!(id, 0);
        assert!(kind <= 3, "kind {kind} has no window");
        let (mw, mh) = (dw.max(16), dh.max(16));
        assert!(rect.width >= 16 && rect.width <= mw && rect.height >= 16 && rect.height <= mh);
        assert!(u64::from(rect.x) + u64::from(rect.width) <= u64::from(mw));
        assert!(u64::from(rect.y) + u64::from(rect.height) <= u64::from(mh));
    }
    assert!(accepted > 25_000, "the generator reaches the accepting path: {accepted}");
}

#[test]
fn boundary_bodies() {
    assert!(decode(&[], 1920, 1080).is_none());
    let good = body(1, 0, 10, 10, 300, 200);
    assert!(decode(&good, 1920, 1080).is_some());
    assert!(decode(&good[..WINDOW_OPEN_REQ_LEN - 1], 1920, 1080).is_none());
    let mut long = good.clone();
    long.push(0);
    assert!(decode(&long, 1920, 1080).is_none());
    assert!(decode(&body(0, 0, 10, 10, 300, 200), 1920, 1080).is_none(), "id zero");
    assert!(decode(&body(1, 0, 10, 10, 0, 200), 1920, 1080).is_none(), "width zero");
    assert!(decode(&body(1, 0, 10, 10, 300, 0), 1920, 1080).is_none(), "height zero");
    for kind in 0..=3 {
        assert!(decode(&body(1, kind, 0, 0, 1, 1), 1920, 1080).is_some(), "kind {kind}");
    }
    assert!(decode(&body(1, 4, 0, 0, 1, 1), 1920, 1080).is_none(), "kind past the last");
    let (_, _, r) = decode(&body(1, 0, u32::MAX, u32::MAX, u32::MAX, u32::MAX), 1920, 1080)
        .expect("the largest numbers clamp onto the display");
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 1920, 1080));
}
