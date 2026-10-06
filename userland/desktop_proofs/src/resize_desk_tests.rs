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

//! A window resized from its frame, end to end: the drag on the border, the
//! app's new surface at the same origin, the window manager's rect. Before,
//! the window manager refused any size that overlapped another window (and
//! cascaded windows always do) after the app had drawn it, and moved the
//! origin of one that ran past the screen's edge; either way the window a
//! press reached was not the one on screen.

use super::desk::Desk;

const TERMINAL: u32 = 10;
const EDITOR: u32 = 20;

/// Press the bottom right corner's resize band of `pid`'s window, drag the
/// pointer by (dx, dy) and release.
fn drag_corner(d: &mut Desk, pid: u32, dx: i32, dy: i32) {
    let a = d.app(pid);
    let (x, y) = (a.x + a.w - 2, a.y + a.h - 2);
    d.press(x, y);
    let (tx, ty) = ((x as i32 + dx) as u32, (y as i32 + dy) as u32);
    d.motion(tx, ty);
    d.release(tx, ty);
}

fn wm_rect(d: &Desk, pid: u32) -> (u32, u32, u32, u32) {
    let a = d.app(pid);
    let r = d.windows.find(pid, a.window_id).expect("open").rect;
    (r.x, r.y, r.width, r.height)
}

fn app_rect(d: &Desk, pid: u32) -> (u32, u32, u32, u32) {
    let a = d.app(pid);
    (a.x, a.y, a.w, a.h)
}

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
