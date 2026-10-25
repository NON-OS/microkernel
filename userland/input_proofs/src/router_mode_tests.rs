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

//! H-09: the compositor's size notice moves the cursor's edges at once, and
//! names the op the router reads it by.

use super::router_display_tests::{abs, shell_saw};
use crate::router_desk::*;

#[test]
fn a_mode_change_moves_the_edges_at_once_while_the_router_would_still_ask_the_old() {
    // The router last asked at 1024x768 and would not ask again for 2 s; the
    // compositor's notice (NIRS 0x0006) is all that says 1920x1080 now.
    let mut ctx = desk(1024, 768);
    assert!(ctx.cursor.adopt(1920, 1080));
    for p in [(400, 300), (1600, 900), (855, 620), (1919, 1079), (0, 0)] {
        abs(&mut ctx, p, 1920, 1080);
        assert_eq!(shell_saw(), Some((p.0 as i32, p.1 as i32)), "{p:?}");
    }
}

#[test]
fn a_size_with_a_zero_side_changes_nothing() {
    let mut ctx = desk(1920, 1080);
    assert!(!ctx.cursor.adopt(0, 1080) && !ctx.cursor.adopt(1920, 0));
    abs(&mut ctx, (1600, 900), 1920, 1080);
    assert_eq!(shell_saw(), Some((1600, 900)));
}

#[test]
fn the_op_is_the_one_the_compositor_sends() {
    assert_eq!(crate::router_protocol::OP_DISPLAY_SIZE, 0x0006);
    let sender = include_str!("../../compositor/src/server/display_notice.rs");
    assert!(sender.contains("const OP_DISPLAY_SIZE: u16 = 0x0006;"));
}
