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

//! The router asks the window manager what is under a pointer that hovers no
//! window on every fourth motion only, a cap on synchronous asks. Two events
//! ask whatever the count: the one that leaves a window, which may be the
//! first into another, and, once the input stops, the last one skipped, so the
//! window the pointer rests over always has the hover.

use nonos_libc::INPUT_KIND_POINTER_ABS;

use crate::clients::wm::topmost_asks;
use crate::route::settle_hover;
use crate::router_desk::*;

const A: u32 = 20;
const B: u32 = 21;

fn hovers(rx: &[Rx]) -> Vec<(u32, i32, i32)> {
    rx.iter()
        .filter(|r| r.kind == INPUT_KIND_POINTER_ABS && r.pid != SHELL)
        .map(|r| (r.pid, r.x, r.y))
        .collect()
}

#[test]
fn hover_first_the_resting_place_is_asked_when_input_stops() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 400, 400));
    point(&mut ctx, 100, 100);
    assert_eq!(hovers(&sent()), [], "skipped by the count");
    settle_hover(&mut ctx);
    assert_eq!(hovers(&sent()), [(A, 100, 100)], "asked at rest");
    let asks = topmost_asks();
    settle_hover(&mut ctx);
    assert_eq!(topmost_asks(), asks, "once a pause");
}

#[test]
fn hover_first_leaving_a_window_asks_at_once() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (0, 0, 200, 400));
    window(&mut ctx, B, (200, 0, 200, 400));
    point(&mut ctx, 100, 100);
    settle_hover(&mut ctx);
    sent();
    point(&mut ctx, 250, 100);
    assert_eq!(hovers(&sent()), [(A, -1, -1), (B, 50, 100)], "into B on the first event");
}

#[test]
fn hover_first_keeps_the_cap_while_the_pointer_moves() {
    let mut ctx = desk(800, 600);
    window(&mut ctx, A, (600, 400, 100, 100));
    let before = topmost_asks();
    for i in 0..16 {
        point(&mut ctx, 10 + i * 10, 10);
    }
    assert_eq!(topmost_asks() - before, 4, "one ask in four over no window");
    settle_hover(&mut ctx);
    assert_eq!(topmost_asks() - before, 4, "the last move was asked: nothing owed");
}
