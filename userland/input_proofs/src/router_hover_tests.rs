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

//! Hover motion goes to the window drawn under the pointer, in that window's
//! frame, even when the windows change under a pointer that stays inside the
//! one it last hovered.

use nonos_libc::INPUT_KIND_POINTER_ABS;

use crate::clients::world;
use crate::router_desk::*;
use crate::state::Context;

const A: u32 = 20;
const B: u32 = 21;

fn hovers(rx: &[Rx]) -> Vec<(u32, i32, i32)> {
    rx.iter()
        .filter(|r| r.kind == INPUT_KIND_POINTER_ABS && r.pid != SHELL)
        .map(|r| (r.pid, r.x, r.y))
        .collect()
}

/// Wiggle the pointer around (x, y) until the router has asked the window
/// manager what is under it; the hover is then cached.
fn settle(ctx: &mut Context, x: u32, y: u32) {
    for i in 0..4 {
        point(ctx, x + i % 2, y);
    }
    sent();
}

#[test]
fn a_window_opened_under_a_resting_pointer_gets_the_hover() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    settle(&mut ctx, 100, 100);
    window(&mut ctx, B, (50, 50, 200, 200));
    nonos_libc::advance_ms(1_000);
    point(&mut ctx, 110, 110);
    assert_eq!(hovers(&sent()), [(A, -1, -1), (B, 60, 60)]);
}

#[test]
fn a_hovered_window_that_moved_is_hovered_in_its_new_frame() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    settle(&mut ctx, 100, 100);
    world::move_to(A, 50, 0);
    nonos_libc::advance_ms(1_000);
    point(&mut ctx, 110, 110);
    assert_eq!(hovers(&sent()), [(A, 60, 110)]);
}

#[test]
fn a_hover_inside_its_window_is_not_asked_again_on_every_motion() {
    // The cached hover is what keeps a synchronous window manager query off
    // every motion event; within the recheck interval it is trusted.
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    settle(&mut ctx, 100, 100);
    window(&mut ctx, B, (50, 50, 200, 200));
    point(&mut ctx, 110, 110);
    assert_eq!(hovers(&sent()), [(A, 110, 110)]);
}
