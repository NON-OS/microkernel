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
use crate::scene::Layer;
use crate::state::cursor::CursorState;
use crate::state::cursor_pick::{reach, reshaped, shape_at, HOT_REACH, SHAPE_BOX};

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

/// The pointer still over a window's edge, the resize arrow drawn, and the
/// window moved away from under it: the shape there is the arrow now, with
/// no motion and no OP_CURSOR_SHAPE. Only the window's rect was repainted,
/// so the part of the resize arrow outside it stayed beside the new arrow.
/// The pointer's box is owed.
#[test]
fn a_shape_changed_under_a_still_pointer_repaints_its_box() {
    let at = CursorState { x: 400, y: 150, visible: true };
    let mut edge = layer(40, 0, 0, 401, 300, 2);
    edge.cursor = Some(EW_CODE);
    let drawn = shape_at([edge].iter(), at.x, at.y);
    assert_eq!(drawn, EW_CODE);
    let moved = Layer { x: 600, ..edge };
    let want = shape_at([moved].iter(), at.x, at.y);
    assert_eq!(want, ARROW_CODE);
    let owed = reshaped(at, false, (drawn, want), (1920, 1080));
    let r = owed.expect("the pointer's box");
    let want = reach(400, 150, 1920, 1080);
    assert_eq!((r.x, r.y, r.width, r.height), (want.x, want.y, want.width, want.height));
}

#[test]
fn nothing_is_owed_for_the_same_shape_a_hidden_pointer_or_the_plane() {
    let at = CursorState { x: 10, y: 10, visible: true };
    assert!(reshaped(at, false, (EW_CODE, EW_CODE), (640, 480)).is_none());
    assert!(reshaped(at, true, (EW_CODE, ARROW_CODE), (640, 480)).is_none(), "the plane's");
    let hidden = CursorState { visible: false, ..at };
    assert!(reshaped(hidden, false, (EW_CODE, ARROW_CODE), (640, 480)).is_none());
}

#[test]
fn every_frame_follows_the_composited_pointers_shape() {
    const TICK: &str = include_str!("../../../../compositor/src/frame_pacer/tick.rs");
    const FOLLOW: &str = include_str!("../../../../compositor/src/frame_pacer/sw_cursor.rs");
    let begin = TICK.find("ctx.trace.begin();").expect("a frame");
    let follow = TICK.find("super::sw_cursor::follow(ctx);").expect("followed");
    let drew = TICK.find("let drew = !ctx.damage.is_empty();").expect("drawn");
    assert!(begin < follow && follow < drew, "followed before the frame's damage is read");
    assert!(FOLLOW.contains("if ctx.hw_cursor() || !c.visible {"));
    assert!(FOLLOW.contains("ctx.sw_cursor_code = want;"));
}
