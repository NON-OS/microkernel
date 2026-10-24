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

//! Two windows on the desk, a corner dragged, and where each is drawn.

use super::super::super::desk::Desk;

pub(super) const TERMINAL: u32 = 10;

pub(super) const EDITOR: u32 = 20;

/// Press the bottom right corner's resize band of `pid`'s window, drag the
/// pointer by (dx, dy) and release.
pub(super) fn drag_corner(d: &mut Desk, pid: u32, dx: i32, dy: i32) {
    let a = d.app(pid);
    let (x, y) = (a.x + a.w - 2, a.y + a.h - 2);
    d.press(x, y);
    let (tx, ty) = ((x as i32 + dx) as u32, (y as i32 + dy) as u32);
    d.motion(tx, ty);
    d.release(tx, ty);
}

pub(super) fn wm_rect(d: &Desk, pid: u32) -> (u32, u32, u32, u32) {
    let a = d.app(pid);
    let r = d.windows.find(pid, a.window_id).expect("open").rect;
    (r.x, r.y, r.width, r.height)
}

pub(super) fn app_rect(d: &Desk, pid: u32) -> (u32, u32, u32, u32) {
    let a = d.app(pid);
    (a.x, a.y, a.w, a.h)
}
