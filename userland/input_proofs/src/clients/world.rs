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

//! The desk the router's peers answer from: the display size, the shell's
//! pid, the windows top first, and which one has focus.

use std::cell::RefCell;

#[derive(Clone, Copy, Debug)]
pub struct Win {
    pub pid: u32,
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Win {
    pub fn contains(&self, px: u32, py: u32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }
}

#[derive(Default)]
pub struct World {
    pub display: (u32, u32),
    pub shell: u32,
    /// Top first, as the window manager's hit test walks them.
    pub windows: Vec<Win>,
    pub focus: u32,
}

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::default());
}

pub fn with<R>(f: impl FnOnce(&mut World) -> R) -> R {
    WORLD.with(|w| f(&mut w.borrow_mut()))
}

/// A fresh desk of `width` by `height` with the shell up as `shell`.
pub fn reset(width: u32, height: u32, shell: u32) {
    with(|w| *w = World { display: (width, height), shell, windows: Vec::new(), focus: 0 });
}

/// Open a window on top of the others, with focus, as window_open does.
pub fn open(pid: u32, x: u32, y: u32, w: u32, h: u32) {
    with(|world| {
        world.windows.insert(0, Win { pid, id: 0x5749_4E00 | pid, x, y, w, h });
        world.focus = pid;
    });
}

/// Move `pid`'s window to (x, y), as a drag or the app itself does.
pub fn move_to(pid: u32, x: u32, y: u32) {
    with(|world| {
        if let Some(win) = world.windows.iter_mut().find(|win| win.pid == pid) {
            win.x = x;
            win.y = y;
        }
    });
}

pub fn close(pid: u32) {
    with(|world| {
        world.windows.retain(|win| win.pid != pid);
        if world.focus == pid {
            world.focus = world.windows.first().map_or(0, |win| win.pid);
        }
    });
}

pub fn set_focus(pid: u32) {
    with(|world| world.focus = pid);
}

pub fn focus() -> u32 {
    with(|world| world.focus)
}
