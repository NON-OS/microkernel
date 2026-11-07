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

//! OP_POPUP_OPEN's body: each field where the wire puts it, and every value
//! xdg_positioner does not have refused.

use crate::window::popup::{decode, encode, POPUP_OPEN_REQ_LEN};

fn body(words: [u32; 14]) -> Vec<u8> {
    words.iter().flat_map(|w| w.to_le_bytes()).collect()
}

const GOOD: [u32; 14] =
    [9, 1, (-4i32) as u32, 30, 80, 24, 200, 300, 6, 8, 63, 5, (-3i32) as u32, 1];

#[test]
fn a_popup_open_reads_as_the_wire_defines() {
    let ask = decode(&body(GOOD)).expect("a sound request");
    assert_eq!((ask.window_id, ask.parent_id, ask.grab), (9, 1, true));
    let p = ask.positioner;
    assert_eq!((p.anchor_x, p.anchor_y, p.anchor_w, p.anchor_h), (-4, 30, 80, 24));
    assert_eq!((p.width, p.height, p.anchor, p.gravity, p.adjust), (200, 300, 6, 8, 63));
    assert_eq!((p.offset_x, p.offset_y), (5, -3));
    assert_eq!(body(GOOD).len(), POPUP_OPEN_REQ_LEN);
}

#[test]
fn every_value_out_of_range_is_refused() {
    let bad: [(usize, u32); 10] = [
        (0, 0),   // no popup id
        (1, 0),   // no parent
        (1, 9),   // its own parent
        (6, 0),   // no width
        (7, 0),   // no height
        (8, 9),   // no such anchor
        (9, 9),   // no such gravity
        (10, 64), // no such adjustment
        (13, 2),  // an unknown flag
        (13, u32::MAX),
    ];
    for (at, v) in bad {
        let mut w = GOOD;
        w[at] = v;
        assert!(decode(&body(w)).is_none(), "word {at} = {v}");
    }
}

#[test]
fn a_body_of_another_length_is_refused() {
    let b = body(GOOD);
    assert!(decode(&b[..POPUP_OPEN_REQ_LEN - 4]).is_none());
    let mut long = b.clone();
    long.extend_from_slice(&[0; 4]);
    assert!(decode(&long).is_none());
}

#[test]
fn the_reply_carries_the_place_and_the_place_on_the_parent() {
    use crate::geometry::positioner::Placed;
    use crate::geometry::Rect;
    let p = Placed { rect: Rect { x: 1, y: 2, width: 3, height: 4 }, rel_x: -5, rel_y: 6 };
    let out = encode(&p);
    let w: Vec<u32> = out.chunks(4).map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]])).collect();
    assert_eq!(w, vec![1, 2, 3, 4, (-5i32) as u32, 6]);
}
