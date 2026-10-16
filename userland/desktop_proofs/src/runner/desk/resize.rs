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

//! A window resized or moved, as the window manager and the app settle it.

use super::types::Desk;
use crate::geometry::resize::resized;
use crate::geometry::{clamp_to_display, Rect};

impl Desk {
    /// apply_resize: the size settled against the opening size and the room
    /// the display leaves right of and below the origin, the new surface
    /// submitted at the same origin, then window_resize as the window manager
    /// takes it.
    pub fn apply_resize(&mut self, i: usize, w: u32, h: u32) {
        let (x, y, pid, wid) =
            (self.apps[i].x, self.apps[i].y, self.apps[i].pid, self.apps[i].window_id);
        let bound = Some(super::super::min_size::room((self.width, self.height), (x, y)));
        let opened = self.apps[i].opened;
        let (w, h) = super::super::min_size::settle((w, h), opened, bound);
        if (w, h) == (self.apps[i].w, self.apps[i].h) {
            return;
        }
        // reopen_surface: the new surface at the same origin, in the layer the
        // window has (it was pressed, so it is on top of its band already).
        self.submit(pid, x, y, w, h);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        wm.rect = resized(wm.rect, w, h, self.width, self.height);
        self.apps[i].w = w;
        self.apps[i].h = h;
        self.frame();
    }

    pub(super) fn apply_move(&mut self, i: usize, to_x: u32, to_y: u32) {
        let (w, h, pid, wid) =
            (self.apps[i].w, self.apps[i].h, self.apps[i].pid, self.apps[i].window_id);
        let nx = to_x.min(self.width.saturating_sub(w));
        let ny = to_y.min(self.height.saturating_sub(h));
        if (nx, ny) == (self.apps[i].x, self.apps[i].y) {
            return;
        }
        self.submit(pid, nx, ny, w, h);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        wm.rect =
            clamp_to_display(Rect { x: nx, y: ny, width: w, height: h }, self.width, self.height);
        self.apps[i].x = nx;
        self.apps[i].y = ny;
        self.apps[i].moves += 1;
    }
}
