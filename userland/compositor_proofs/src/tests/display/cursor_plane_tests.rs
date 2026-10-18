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

//! The arrow a device's cursor plane shows, drawn into a primary's slot.
use crate::frame_pacer::cursor::{FILL, OUTLINE};
use crate::frame_pacer::cursor_shape::{draw_shape, shape_side, SIDE, SLOT};

fn px(slot: &[u8], x: usize, y: usize) -> u32 {
    let at = (y * SIDE + x) * 4;
    u32::from_le_bytes(slot[at..at + 4].try_into().unwrap())
}

#[test]
fn the_arrow_is_drawn_on_a_clear_slot_its_tip_at_the_corner() {
    let mut slot = vec![0xEEu8; SLOT];
    assert_eq!(draw_shape(&mut slot, 1), Some(()));
    assert_eq!((px(&slot, 0, 0), px(&slot, 1, 2), px(&slot, 2, 2)), (OUTLINE, FILL, OUTLINE));
    assert_eq!((px(&slot, 1, 0), px(&slot, 63, 63)), (0, 0), "transparent, not left over");
    assert_eq!(shape_side(1), (11, 19));
    assert_eq!(draw_shape(&mut slot[..SLOT - 1], 1), None, "a short slot is refused");
}

#[test]
fn a_scaled_screen_gets_a_scaled_arrow_that_still_fits() {
    let mut slot = vec![0u8; SLOT];
    draw_shape(&mut slot, 2);
    assert_eq!((px(&slot, 1, 1), px(&slot, 0, 4), px(&slot, 3, 5)), (OUTLINE, OUTLINE, FILL));
    assert_eq!(shape_side(2), (22, 38));
    draw_shape(&mut slot, 9);
    assert_eq!(shape_side(9), (33, 57), "three a pixel at most: 19 rows in 64");
    assert!((57..SIDE).all(|y| (0..SIDE).all(|x| px(&slot, x, y) == 0)));
}
