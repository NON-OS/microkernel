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

//! A window grown over another is hit where it is drawn.

use super::super::super::desk::Desk;
use super::rig::{app_rect, drag_corner, wm_rect, EDITOR, TERMINAL};

#[test]
fn a_window_grown_over_another_is_hit_where_it_is_drawn() {
    let mut d = Desk::new(1280, 720);
    d.open(TERMINAL, (500, 100, 400, 300), 0);
    d.open(EDITOR, (100, 80, 380, 300), 0);
    drag_corner(&mut d, EDITOR, 200, 150);
    assert_eq!(app_rect(&d, EDITOR), (100, 80, 580, 450), "the editor grew over the terminal");
    assert_eq!(wm_rect(&d, EDITOR), app_rect(&d, EDITOR));
    assert_eq!(d.hit(600, 400), EDITOR, "a press on the part it grew into reaches it");
    d.assert_consistent("grown over the terminal");

    drag_corner(&mut d, EDITOR, -150, -100);
    assert_eq!(wm_rect(&d, EDITOR), app_rect(&d, EDITOR));
    d.assert_consistent("shrunk back while still over the terminal");
}
