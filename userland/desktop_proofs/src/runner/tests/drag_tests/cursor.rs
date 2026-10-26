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
//! As the pointer moves over the frame the drag keeps the shape it wants
//! there, and app_skeleton asks the compositor for it once a change
//! (window_ops/tell_cursor.rs), not once a move.

use super::super::super::desk::event;
use super::super::super::drag::{self, DragState};
use super::rig::{H, W};
use crate::input::InputKind;
use nonos_toolkit::decorations::{ARROW_CURSOR, EW_CURSOR, NWSE_CURSOR};

const TELL: &str = include_str!("../../../../../app_skeleton/src/runner/window_ops/tell_cursor.rs");

fn over(s: &mut DragState, x: i32, y: i32) -> u8 {
    drag::handle(s, (W, H), (100, 100), false, &event(InputKind::PointerAbs, x, y));
    s.cursor.0
}

#[test]
fn the_shape_follows_the_edge_under_the_pointer() {
    let mut s = DragState::new();
    assert_eq!(over(&mut s, 3, H as i32 / 2), EW_CURSOR, "the left edge");
    assert_eq!(over(&mut s, 3, 3), NWSE_CURSOR, "the top-left corner");
    assert_eq!(over(&mut s, W as i32 / 2, H as i32 / 2), ARROW_CURSOR, "the app's own area");
}

#[test]
fn the_compositor_is_asked_once_a_change() {
    assert!(TELL.contains("if want == told {\n        return;\n    }"), "nothing when unchanged");
    assert!(TELL.contains("booted.drag.cursor.1 = want;"), "remembered once asked");
}
