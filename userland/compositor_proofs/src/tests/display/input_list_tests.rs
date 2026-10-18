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

//! INPUT_SOURCES from the driver's readings: a display offers the inputs
//! its capabilities list and shows the one it reads now; a refusal, a short
//! reading or one listing nothing offers none; three displays at most.

use crate::display::input_list::{encode, offered, request, Offered, INPUT_SOURCES_LEN};

pub(super) fn reading(status: i32, now: u16, listed: &[u8]) -> Vec<u8> {
    let mut r = status.to_le_bytes().to_vec();
    r.extend_from_slice(&now.to_le_bytes());
    r.extend_from_slice(&0x12u16.to_le_bytes());
    r.push(listed.len() as u8);
    r.extend_from_slice(listed);
    r
}

#[test]
fn a_reading_offers_the_listed_inputs_and_the_one_shown() {
    let id = 0x8000_0001;
    assert_eq!(request(id), [23, 1, 0, 0, 0x80, 0x60]);
    let o = offered(id, &reading(0, 0x0F, &[0x0F, 0x11])).unwrap();
    assert_eq!((o.identity, o.now, o.n, &o.inputs[..2]), (id, 0x0F, 2, &[0x0F, 0x11][..]));
    assert_eq!(
        offered(id, &reading(-38, 0x0F, &[0x0F])).or(offered(id, &reading(0, 1, &[]))),
        None
    );
    let many: Vec<u8> = (1..=12).collect();
    assert_eq!(offered(id, &reading(0, 3, &many)).unwrap().inputs, [1, 2, 3, 4, 5, 6, 7, 8]);
}

#[test]
fn the_answer_holds_three_displays_in_order() {
    let d = |i: u32| Offered { identity: 0x8000_0000 | i, now: 0x11, inputs: [i as u8; 8], n: 3 };
    let out = encode(&[d(0), d(1), d(2), d(3)]);
    assert_eq!(out.len(), INPUT_SOURCES_LEN);
    assert_eq!(out[..4], 3u32.to_le_bytes(), "the fourth is left out");
    for (i, e) in out[4..].chunks_exact(16).enumerate() {
        assert_eq!(e[..4], (0x8000_0000 | i as u32).to_le_bytes());
        assert_eq!((e[4], e[5], e[6], e[7]), (0x11, 0, 3, 0));
        assert_eq!(e[8..], [i as u8; 8]);
    }
    assert_eq!(encode(&[]), [0; INPUT_SOURCES_LEN]);
}
