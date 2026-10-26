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
//! A window placed from its left or top edge, as app_skeleton's
//! window_ops/place_window.rs settles it and the window manager's
//! OP_WINDOW_PLACE takes it: one surface, one rect.

use super::types::Desk;
use crate::geometry::resize::placed;
use nonos_toolkit::decorations::{keep_far_edges, Placed};

impl Desk {
    pub(super) fn apply_place(&mut self, i: usize, p: Placed) {
        let a = &self.apps[i];
        let (bx, by, bw, bh, pid, wid) = (a.x, a.y, a.w, a.h, a.pid, a.window_id);
        let bound = Some(super::super::min_size::room((self.width, self.height), (p.x, p.y)));
        let (w, h) = super::super::min_size::settle((p.w, p.h), a.opened, bound);
        let base = Placed { x: bx, y: by, w: bw, h: bh };
        let Placed { x, y, .. } = keep_far_edges(p, base, w, h);
        if (x, y, w, h) == (bx, by, bw, bh) {
            return;
        }
        self.submit(pid, x, y, w, h);
        let wm = self.windows.find_mut(pid, wid).expect("open");
        wm.rect = placed(x, y, w, h, self.width, self.height);
        let a = &mut self.apps[i];
        (a.x, a.y, a.w, a.h) = (x, y, w, h);
        self.frame();
    }
}
