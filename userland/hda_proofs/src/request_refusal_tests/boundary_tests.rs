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

//! Frames at each edge: empty, one byte short, every op, the largest body.

use super::frame::{check, frame, BODY_MAX, LAST_OP};
use crate::protocol::{refused, HDR_LEN};

#[test]
fn boundary_frames() {
    assert!(!check(&[]));
    let short = frame(1, 7, 0, &[]);
    assert!(!check(&short[..HDR_LEN - 1]), "one byte short of a header");
    assert!(check(&short), "a bare header");
    // The decode leaves the length field to the loop and its handlers.
    assert!(check(&frame(1, 9, 5, &[1, 2, 3, 4])));
    assert!(check(&frame(1, 11, u32::MAX, &[])));
    for op in (0..=LAST_OP + 1).chain([u16::MAX]) {
        assert!(check(&frame(op, u32::from(op), 0, &[])), "op {op}");
    }
    assert!(check(&frame(1, 13, BODY_MAX as u32, &vec![0xA5u8; BODY_MAX])), "the largest body");
    let mut bad = frame(1, 15, 0, &[]);
    bad[0] ^= 0xFF;
    assert!(!check(&bad), "the wrong magic");
    assert_eq!(u64::from(refused(&bad).request_id), 15);
}
