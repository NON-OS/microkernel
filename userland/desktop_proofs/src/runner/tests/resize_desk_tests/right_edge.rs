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

//! A window widened to the right edge keeps its origin and its border.

use super::super::super::desk::Desk;
use super::rig::{app_rect, drag_corner, wm_rect, TERMINAL};

#[test]
fn a_window_widened_to_the_right_edge_keeps_its_origin_and_its_border_on_screen() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (800, 100, 400, 300), 0);
    drag_corner(&mut d, TERMINAL, 400, 0);
    let (x, _, w, _) = app_rect(&d, TERMINAL);
    assert_eq!(x, 800, "the origin stays");
    assert_eq!(x + w, 1280, "the right border stops at the screen's edge, not past it");
    assert_eq!(wm_rect(&d, TERMINAL), app_rect(&d, TERMINAL), "and the window manager agrees");
    d.assert_consistent("widened to the edge");
}
