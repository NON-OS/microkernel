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
//! The left and top edges and the top-left corner resize too, as one
//! placing: the edge dragged follows the pointer, the opposite edge stays,
//! and the window manager holds the same rect as the app.

use super::super::super::desk::Desk;
use super::rig::{app_rect, wm_rect, TERMINAL};

/// A press `inset` px inside the window's corner or edge at (fx, fy) of its
/// size, dragged by (dx, dy).
fn drag_at(d: &mut Desk, (fx, fy): (u32, u32), (dx, dy): (i32, i32)) {
    let a = d.app(TERMINAL);
    let (x, y) = (a.x + 2 + (a.w - 4) * fx / 2, a.y + 2 + (a.h - 4) * fy / 2);
    d.press(x, y);
    let (tx, ty) = ((x as i32 + dx) as u32, (y as i32 + dy) as u32);
    d.motion(tx, ty);
    d.release(tx, ty);
}

#[test]
fn the_left_edge_moves_with_the_pointer_and_the_right_one_stays() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (400, 200, 600, 400), 0);
    drag_at(&mut d, (0, 1), (-150, 0));
    assert_eq!(app_rect(&d, TERMINAL), (250, 200, 750, 400), "wider to the left");
    assert_eq!(wm_rect(&d, TERMINAL), app_rect(&d, TERMINAL), "and the window manager agrees");
    d.assert_consistent("the left edge");
}

#[test]
fn the_top_left_corner_moves_both_edges_and_keeps_the_opposite_corner() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (400, 200, 600, 400), 0);
    drag_at(&mut d, (0, 0), (100, 60));
    assert_eq!(app_rect(&d, TERMINAL), (500, 260, 500, 340), "smaller from the top left");
    assert_eq!(wm_rect(&d, TERMINAL), app_rect(&d, TERMINAL));
    d.assert_consistent("the top-left corner");
}

#[test]
fn a_left_edge_dragged_past_the_least_size_stops_and_the_right_one_stays() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (400, 200, 600, 400), 0);
    drag_at(&mut d, (0, 1), (900, 0));
    let (x, _, w, _) = app_rect(&d, TERMINAL);
    assert_eq!(x + w, 1000, "the right edge has not moved");
    assert!(w < 600, "it shrank, to the least it may be");
    d.assert_consistent("past the least size");
}
