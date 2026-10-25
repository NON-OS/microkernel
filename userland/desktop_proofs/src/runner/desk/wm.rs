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

//! A change through the window manager: minimise and restore.

use super::types::Desk;
use crate::window::full_screen::covers_dock_at;
use crate::window::Visibility;
use crate::z_order::raise;

impl Desk {
    /// A window manager change to `pid`'s window, then the full-screen
    /// notification the window manager sends when whether it covers the
    /// dock's band changed (server/full_screen_notify.rs), as the shell
    /// takes it (server/wm_notify.rs), then the shell's sync.
    pub(super) fn wm_change(&mut self, pid: u32, change: impl FnOnce(&mut Self, u32)) {
        let wid = 0x5749_4E00 | pid;
        let was = covers_dock_at(&self.windows, pid, wid);
        change(self, wid);
        let now = covers_dock_at(&self.windows, pid, wid);
        if now != was && self.shell != 0 {
            crate::taskbar::set_full_screen(&mut self.taskbar, pid, wid, now);
        }
        self.shell_sync();
    }

    /// The window's minimise button: window_minimize, then its layer removed.
    pub fn minimize(&mut self, pid: u32) {
        self.wm_change(pid, |d, wid| {
            d.windows.find_mut(pid, wid).expect("open").visibility = Visibility::Minimized;
            d.drop_layer(pid);
        });
    }

    /// The dock raising a minimised window: window_restore, then the app's
    /// layer submitted again where it was.
    pub fn restore_window(&mut self, pid: u32) {
        self.wm_change(pid, |d, wid| {
            d.windows.find_mut(pid, wid).expect("open").visibility = Visibility::Visible;
            let _ = raise(&mut d.windows, &mut d.z, pid, wid);
            let a = d.app(pid);
            let (x, y, w, h) = (a.x, a.y, a.w, a.h);
            d.submit(pid, x, y, w, h);
        });
    }
}
