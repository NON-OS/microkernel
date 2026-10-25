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

//! The shell following the window manager's word.

use super::consts::DOCK_WINDOW_ID;
use super::types::Desk;
use crate::geometry::Rect;
use crate::window::{Kind, Visibility, Window};

impl Desk {
    /// server/dock_sync.rs: repaint and commit the dock's area, and open or
    /// close its window, as the rule asks.
    pub fn shell_sync(&mut self) {
        let work = crate::taskbar::dock_work(&self.taskbar);
        if work.paint {
            self.paint_dock();
            let area = self.dock_area();
            self.damage.accumulate(area);
            self.dock_commits.push(area);
            self.taskbar.drawn = self.taskbar.visible;
        }
        match work.window {
            Some(true) => {
                let (x, y, width, height) = self.dock;
                let window = Window {
                    owner_pid: self.shell,
                    window_id: DOCK_WINDOW_ID,
                    rect: Rect { x, y, width, height },
                    kind: Kind::Popup,
                    visibility: Visibility::Visible,
                    z: self.z.allocate(),
                    in_use: true,
                    full_screen: false,
                };
                self.windows.insert(window).expect("room for the dock");
                self.taskbar.window_open = true;
            }
            Some(false) => {
                let _ = self.windows.remove(self.shell, DOCK_WINDOW_ID);
                self.taskbar.window_open = false;
            }
            None => {}
        }
        self.frame();
    }
}
