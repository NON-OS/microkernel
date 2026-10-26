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
//! The pointer's shape: drawn from its hot spot inside the damaged box, the
//! topmost window under it choosing, set by a surface's owner alone.

use super::frame_util::layer;
use crate::frame_pacer::cursor_shapes::{
    shape, ARROW_CODE, EW_CODE, NESW_CODE, NS_CODE, NWSE_CODE,
};
use crate::state::cursor_pick::{shape_at, HOT_REACH, SHAPE_BOX};

const CODES: [u8; 5] = [ARROW_CODE, EW_CODE, NS_CODE, NWSE_CODE, NESW_CODE];

#[test]
fn each_shape_is_drawn_from_its_hot_spot_inside_the_damaged_box() {
    for code in CODES {
        let s = shape(code);
        let (hx, hy) = s.hot;
        let at = s.rows[hy as usize].get(hx as usize).copied();
        let want = if code == ARROW_CODE { b'X' } else { b'o' };
        assert_eq!(at, Some(want), "shape {code}: the hot spot is its tip or its middle");
        let w = s.rows.iter().map(|r| r.len()).max().unwrap() as u32;
        assert!(hx <= HOT_REACH && hy <= HOT_REACH, "shape {code} reaches back too far");
        assert!(
            w - hx <= SHAPE_BOX - HOT_REACH && s.rows.len() as u32 - hy <= SHAPE_BOX - HOT_REACH
        );
    }
    assert!(std::ptr::eq(shape(9), shape(ARROW_CODE)), "an unknown code is the arrow");
}

#[test]
fn the_topmost_window_under_the_pointer_chooses_and_elsewhere_is_the_arrow() {
    let (mut low, mut high) = (layer(40, 0, 0, 400, 300, 2), layer(41, 200, 100, 400, 300, 2));
    (low.cursor, high.cursor) = (Some(EW_CODE), Some(NS_CODE));
    let stack = [low, high];
    assert_eq!(shape_at(stack.iter(), 250, 150), NS_CODE, "the window on top, where both are");
    assert_eq!(shape_at(stack.iter(), 50, 50), EW_CODE, "the one below, where it shows");
    assert_eq!(shape_at(stack.iter(), 700, 500), ARROW_CODE, "over no window");
}
