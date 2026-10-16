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

//! Any request header decodes or is refused, without a panic.

use crate::protocol::{decode_request, HDR_LEN};

use super::frames::{frame, le32, xorshift, MAGIC};

#[test]
fn any_header_decodes_or_is_refused() {
    let mut s = 0x4E4D_4B54_0000_0002u64;
    for _ in 0..200_000 {
        let r = xorshift(&mut s);
        let mut f =
            frame(xorshift(&mut s) as u16, xorshift(&mut s) as u32, xorshift(&mut s) as u32);
        match r % 6 {
            0 => f.truncate((xorshift(&mut s) % (HDR_LEN as u64 + 1)) as usize),
            1 => f[(xorshift(&mut s) % 6) as usize] ^= 1 << (xorshift(&mut s) % 8),
            2 => f.extend((0..xorshift(&mut s) % 64).map(|_| xorshift(&mut s) as u8)),
            _ => {}
        }
        match decode_request(&f) {
            Some(req) => {
                assert!(f.len() >= HDR_LEN);
                assert_eq!((le32(&f, 0), u16::from_le_bytes([f[4], f[5]])), (MAGIC, 1));
                assert_eq!((req.request_id, req.payload_len), (le32(&f, 12), le32(&f, 16)));
            }
            None => assert!(f.len() < HDR_LEN || le32(&f, 0) != MAGIC || f[4..6] != [1, 0]),
        }
    }
    assert!(decode_request(&[]).is_none());
    assert!(decode_request(&frame(1, 2, 0)[..HDR_LEN - 1]).is_none());
    assert!(decode_request(&frame(1, 2, u32::MAX)).is_some(), "the loop bounds the body");
}
