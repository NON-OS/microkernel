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

//! The screen composed, and what it shows and hits at a point.

use super::consts::{CLEAR, SHELL_CHROME_Z};
use super::types::{App, Desk};
use crate::damage::Rect as Damage;
use crate::focus::topmost_hit_at;

impl Desk {
    pub(super) fn paint(&mut self, clip: Damage) {
        let (x1, y1) =
            ((clip.x + clip.width).min(self.width), (clip.y + clip.height).min(self.height));
        let (layers, n) = self.scene.z_sorted_snapshot();
        for y in clip.y..y1 {
            for x in clip.x..x1 {
                let i = (y * self.width + x) as usize;
                let top = layers[..n]
                    .iter()
                    .rfind(|l| {
                        let inside =
                            x >= l.x && x < l.x + l.width && y >= l.y && y < l.y + l.height;
                        let clear = l.z == SHELL_CHROME_Z && self.chrome[i] == CLEAR;
                        inside && !clear
                    })
                    .map_or(0, |l| l.owner_pid);
                self.screen[(y * self.width + x) as usize] = top;
            }
        }
    }

    /// What the screen shows at a point.
    pub fn shown_at(&self, x: u32, y: u32) -> u32 {
        self.screen[(y * self.width + x) as usize]
    }

    /// Who a press at a point would go to: the window hit, or the shell (0
    /// before it is up) where no window is.
    pub fn hit(&self, x: u32, y: u32) -> u32 {
        topmost_hit_at(&self.windows, x, y, self.shell).map_or(self.shell, |h| h.owner_pid)
    }

    pub fn focused(&self) -> u32 {
        self.focus.current().map_or(0, |f| f.owner_pid)
    }

    pub fn app(&self, pid: u32) -> &App {
        self.apps.iter().find(|a| a.pid == pid).expect("open")
    }
}
