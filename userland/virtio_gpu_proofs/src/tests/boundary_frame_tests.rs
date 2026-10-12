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

//! The frames at every edge of the header decode: empty, one byte short of a
//! header, a length one past or short of the bytes, the largest length, every
//! op, the largest body, the wrong magic.

use super::frames::{frame, BODY_MAX, LAST_OP};
use super::refusal::check;
use crate::protocol::{refused, HDR_LEN};

#[test]
fn boundary_frames() {
    assert!(!check(&[]));
    let short = frame(1, 0, 7, 0, &[]);
    assert!(!check(&short[..HDR_LEN - 1]), "one byte short of a header");
    assert!(check(&short), "a bare header");
    assert!(!check(&frame(1, 0, 9, 5, &[1, 2, 3, 4])), "a length field one past the bytes");
    assert!(!check(&frame(1, 0, 9, 3, &[1, 2, 3, 4])), "one short of them");
    assert!(!check(&frame(1, 0, 11, u32::MAX, &[])), "the largest length field");
    for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
        assert!(check(&frame(op, 3, u32::from(op), 0, &[])), "op {op}");
    }
    let big = vec![0xA5u8; BODY_MAX];
    assert!(check(&frame(1, 0, 13, BODY_MAX as u32, &big)), "the largest body");
    let mut bad = frame(1, 0, 15, 0, &[]);
    bad[0] ^= 0xFF;
    assert!(!check(&bad), "the wrong magic");
    assert_eq!(refused(&bad).request_id, 15);
}
