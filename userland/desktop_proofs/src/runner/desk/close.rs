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

//! A window closed, and its layer let go.

use super::types::Desk;
use crate::damage::Rect as Damage;

impl Desk {
    /// The window's close button: its layer gone, window_close, and the
    /// closed event the shell takes as the end of its full screen.
    pub fn close(&mut self, pid: u32) {
        let wid = 0x5749_4E00 | pid;
        self.drop_layer(pid);
        let _ = self.windows.remove(pid, wid);
        self.apps.retain(|a| a.pid != pid);
        if self.shell != 0 {
            crate::taskbar::set_full_screen(&mut self.taskbar, pid, wid, false);
        }
        self.shell_sync();
    }

    pub(super) fn drop_layer(&mut self, pid: u32) {
        let a = self.app(pid);
        let r = Damage { x: a.x, y: a.y, width: a.w, height: a.h };
        self.scene.drop_by_pid(pid);
        self.damage.accumulate(r);
        self.frame();
    }
}
