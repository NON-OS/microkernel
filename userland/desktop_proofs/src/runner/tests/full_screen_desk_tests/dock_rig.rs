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

//! The dock over a full-screen window, on a desk with the shell.

use super::super::super::desk::Desk;
use super::rig::APP;

// The dock over a full-screen window: the shell's rule (the real
// state/taskbar code) driven by the window manager's full-screen
// notifications and the pointer the router mirrors to it, its chrome
// composited over the window.

pub(super) const SHELL: u32 = 5;

pub(super) const OTHER: u32 = 31;

/// The dock on a 1280 by 800 screen: 400 by 64, 16 above the bottom edge.
pub(super) const DOCK: (u32, u32, u32, u32) = (440, 720, 400, 64);

pub(super) fn desk_full_screen() -> Desk {
    let mut d = Desk::new(1280, 800);
    d.open_shell(SHELL, DOCK);
    d.open(APP, (200, 150, 640, 480), 0);
    d.green(APP);
    d
}

/// Every pixel of the dock's area, panel and shadow, shows `pid`.
pub(super) fn dock_area_shows(d: &Desk, pid: u32) -> bool {
    let (x, y, w, h) = DOCK;
    (y - 3..y + h + 3).all(|py| (x - 3..x + w + 3).all(|px| d.shown_at(px, py) == pid))
}
