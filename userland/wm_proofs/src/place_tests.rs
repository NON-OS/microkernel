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
//! Subscribers hear that focus moved whenever it did, by whatever path:
//! A window placed from its left or top edge (OP_WINDOW_PLACE): its origin
//! and size taken together as one rect the display leaves room for, routed
//! to its own handler, and no longer full screen afterwards.

use crate::geometry::resize::placed;
use crate::geometry::Rect;

const DISPATCH: &str = include_str!("../../capsule_wm/src/server/runner/dispatch.rs");
const HANDLER: &str = include_str!("../../capsule_wm/src/server/handlers/window_place.rs");
const OPS: &str = include_str!("../../capsule_wm/src/protocol/ops.rs");

fn rect(r: Rect) -> (u32, u32, u32, u32) {
    (r.x, r.y, r.width, r.height)
}

#[test]
fn a_placed_window_takes_its_origin_and_size_together() {
    assert_eq!(rect(placed(250, 200, 750, 400, 1280, 720)), (250, 200, 750, 400));
}

#[test]
fn a_placed_window_keeps_to_the_display() {
    assert_eq!(
        rect(placed(900, 200, 750, 400, 1280, 720)),
        (900, 200, 380, 400),
        "cut at the right"
    );
    let (_, _, w, h) = rect(placed(10, 10, 1, 1, 1280, 720));
    assert!(w > 1 && h > 1, "never under the least a window may be");
}

#[test]
fn place_is_its_own_op_and_ends_full_screen() {
    assert!(OPS.contains("pub const OP_WINDOW_PLACE: u16 = 0x000F;"));
    assert!(DISPATCH.contains("OP_WINDOW_PLACE => handlers::window_place::handle("));
    for line in
        ["window.rect = placed(x, y, w, h, display_w, display_h);", "window.full_screen = false;"]
    {
        assert!(HANDLER.contains(line), "{line}");
    }
    assert!(
        HANDLER.contains("body.len() != WINDOW_PLACE_REQ_LEN"),
        "a body of another length is refused"
    );
}

const RECT_FOLLOW: &str = include_str!("../../app_skeleton/src/runner/rect_follow.rs");

/// A window coming back from a full-width rect (the terminal's drop-down) to
/// one further right: told as a move then a resize, the move is kept on the
/// display at the old full width and lands at x 0; placed, it lands where it
/// was asked. So the app tells its rect as one place.
#[test]
fn a_rect_followed_is_told_as_one_place_not_a_move_then_a_resize() {
    use crate::geometry::clamp_to_display;
    use crate::geometry::resize::resized;
    let (dw, dh) = (1280, 720);
    let dropped = Rect { x: 0, y: 24, width: dw, height: 300 };
    let moved = clamp_to_display(Rect { x: 400, y: 150, ..dropped }, dw, dh);
    assert_eq!(rect(resized(moved, 800, 500, dw, dh)), (0, 150, 800, 500), "the move lost");
    assert_eq!(rect(placed(400, 150, 800, 500, dw, dh)), (400, 150, 800, 500));
    assert!(RECT_FOLLOW.contains("wm::window_place("), "the rect told in one call");
    for split in ["wm::window_move(", "wm::window_resize("] {
        assert!(!RECT_FOLLOW.contains(split), "{split}");
    }
}
