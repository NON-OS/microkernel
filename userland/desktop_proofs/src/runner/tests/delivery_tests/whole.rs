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

//! Any frame yields an event only when it is whole.

use super::super::super::dispatch::{parse_delivery, DELIVERY_LEN, NINP_HDR_LEN};
use super::frames::{delivery, le32, xorshift};
use crate::wire::NINP_MAGIC;

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
