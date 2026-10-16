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

//! The length-prefixed body readers never read past the body.

use alloc::vec::Vec;

use crate::server::lp::{parse_pair, read_lp_str as read_lp_string, take_lp};

use super::frames::{le32, lp, xorshift};

#[test]
fn the_length_prefixed_readers_stay_inside_the_body() {
    let mut s = 0x4E4D_4B54_0000_0003u64;
    let mut paired = 0usize;
    for _ in 0..200_000 {
        let a: Vec<u8> =
            (0..xorshift(&mut s) % 24).map(|_| b'a' + (xorshift(&mut s) % 26) as u8).collect();
        let b: Vec<u8> = (0..xorshift(&mut s) % 24)
            .map(|_| match xorshift(&mut s) {
                r if r & 15 == 0 => r as u8,
                r => b'a' + (r % 26) as u8,
            })
            .collect();
        let mut body = lp(&mut s, &a);
        body.extend(lp(&mut s, &b));
        if xorshift(&mut s) & 3 == 0 {
            body.truncate((xorshift(&mut s) % (body.len() as u64 + 1)) as usize);
        }
        if let Some(name) = read_lp_string(&body) {
            assert_eq!(name.len(), le32(&body, 0) as usize);
            assert_eq!(name.as_bytes(), &body[4..4 + name.len()]);
        }
        if let Some((first, rest)) = take_lp(&body) {
            assert_eq!(4 + first.len() + rest.len(), body.len());
        }
        if let Some((x, y)) = parse_pair(&body) {
            assert!(4 + x.len() + 4 + y.len() <= body.len());
            paired += 1;
        }
    }
    assert!(paired > 20_000, "the generator reaches whole pairs: {paired}");
    assert!(read_lp_string(&[]).is_none() && take_lp(&[1, 0, 0]).is_none());
    assert!(read_lp_string(&[2, 0, 0, 0, b'a']).is_none(), "a prefix one past the bytes");
    assert_eq!(read_lp_string(&[1, 0, 0, 0, b'a', b'b']), Some("a"), "one short of them");
    assert!(take_lp(&[0xFF, 0xFF, 0xFF, 0xFF]).is_none(), "the largest prefix");
    assert!(read_lp_string(&[1, 0, 0, 0, 0xFF]).is_none(), "not UTF-8");
}
