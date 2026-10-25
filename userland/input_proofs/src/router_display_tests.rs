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

//! H-09: absolute pointer events are placed on the display's own edges, never
//! on a made-up range, and the edges move the moment the compositor says the
//! mode changed, not at the router's next ask.

use nonos_libc::{InputEvent, INPUT_ABS_MAX, INPUT_KIND_POINTER_ABS};

use crate::clients::world;
use crate::route::route_event;
use crate::router_desk::*;
use crate::state::Context;

/// An absolute event at pixel `p` of a `w` by `h` display, as a tablet posts it.
pub(super) fn abs(ctx: &mut Context, p: (u32, u32), w: u32, h: u32) {
    // The least value the router places on pixel v: ceil(v * MAX / (n - 1)).
    let at = |v: u32, n: u32| {
        (((v as i64 * INPUT_ABS_MAX as i64) + (n as i64 - 2)) / (n as i64 - 1)) as i32
    };
    let (x, y) = (at(p.0, w), at(p.1, h));
    let ev = InputEvent {
        kind: INPUT_KIND_POINTER_ABS,
        flags: 0,
        code: 0,
        x,
        y,
        delta_x: 0,
        delta_y: 0,
        timestamp_ns: 0,
    };
    route_event(ctx, &ev);
}

/// Where the shell saw the pointer last, if it was told at all.
pub(super) fn shell_saw() -> Option<(i32, i32)> {
    sent()
        .into_iter()
        .filter(|r| r.pid == SHELL && r.kind == INPUT_KIND_POINTER_ABS)
        .map(|r| (r.x, r.y))
        .next_back()
}

#[test]
fn before_the_display_is_known_a_position_is_held_never_placed_on_a_guess() {
    let mut ctx = desk(1920, 1080);
    ctx.cursor = crate::state::CursorState::new();
    world::with(|w| w.display = (0, 0));
    abs(&mut ctx, (1600, 900), 1920, 1080);
    assert_eq!(shell_saw(), None, "placed with no edges known");
    assert!(!ctx.cursor.configured);
    assert!(ctx.cursor.adopt(1920, 1080));
    assert_eq!((ctx.cursor.x, ctx.cursor.y), (1600, 900), "the held position, on the real edges");
    world::with(|w| w.display = (1920, 1080));
    abs(&mut ctx, (855, 620), 1920, 1080);
    assert_eq!(shell_saw(), Some((855, 620)));
}
