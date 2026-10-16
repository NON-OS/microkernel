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

//! BRIGHTNESS_GET's answer: the first three panels, each its identity and
//! level now (255 unknown), a fixed length however many there are.

use crate::display::brightness_get::{encode, BRIGHTNESS_GET_LEN};
use crate::display::brightness_rule::Panel;

#[test]
fn the_answer_holds_three_panels_and_their_levels() {
    let levels = [1u8, 50, 100];
    let p = |i: u32, now: u8| Panel { identity: 0x8000_0000 | i, now, levels: &levels };
    assert_eq!(encode(&[]), [0; BRIGHTNESS_GET_LEN]);
    let out = encode(&[p(0, 70), p(1, 0xFF), p(2, 5), p(3, 9)]);
    assert_eq!(BRIGHTNESS_GET_LEN, 28);
    assert_eq!(out[..4], 3u32.to_le_bytes(), "the fourth is left out");
    let read = |i: usize| {
        let e = &out[4 + i * 8..12 + i * 8];
        (
            u32::from_le_bytes(e[..4].try_into().unwrap()),
            u32::from_le_bytes(e[4..].try_into().unwrap()),
        )
    };
    assert_eq!(
        [read(0), read(1), read(2)],
        [(0x8000_0000, 70), (0x8000_0001, 255), (0x8000_0002, 5)]
    );
    let one = encode(&[p(4, 33)]);
    assert_eq!(
        (&one[..4], &one[12..]),
        (&1u32.to_le_bytes()[..], &[0; 16][..]),
        "nothing past one"
    );
}
