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

//! Green: a window to full screen and back.

use super::types::Desk;
use crate::geometry::Rect;
use crate::scene_raise::raise_by_pid;
use crate::window::full_screen::{maximize, MAXIMIZE_FLAG_FULL_SCREEN};
use crate::z_order::raise;

impl Desk {
    /// The green button as maximize::toggle presses it: the surface
    /// reopened at chrome::full_screen (or the saved rect), submitted, then
    /// window_maximize with the full-screen flag (or without it).
    pub fn green(&mut self, pid: u32) {
        self.wm_change(pid, |d, _| d.toggle_full_screen(pid));
    }

    pub(super) fn toggle_full_screen(&mut self, pid: u32) {
        let i = self.apps.iter().position(|a| a.pid == pid).expect("open");
        let wid = self.apps[i].window_id;
        let (rect, flags) = match self.apps[i].saved.take() {
            Some(saved) => (saved, 0),
            None => {
                let a = &self.apps[i];
                self.apps[i].saved = Some((a.x, a.y, a.w, a.h));
                (
                    super::super::chrome::full_screen(self.width, self.height),
                    MAXIMIZE_FLAG_FULL_SCREEN,
                )
            }
        };
        let (x, y, w, h) = rect;
        self.submit(pid, x, y, w, h);
        let (dw, dh) = (self.width, self.height);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        maximize(wm, Rect { x, y, width: w, height: h }, flags, dw, dh);
        if raise(&mut self.windows, &mut self.z, pid, wid) == Some(true) {
            if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                self.damage.accumulate(r);
            }
        }
        let a = &mut self.apps[i];
        (a.x, a.y, a.w, a.h) = (x, y, w, h);
        self.frame();
    }
}
