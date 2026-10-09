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

use crate::browser::css::Computed;

use super::ctx::Ctx;
use super::radii::radii;

impl Ctx {
    /* Clip to box `s`'s overflow rectangle `rect`, rounded by the box's own
     * corner radii (less its borders) when the clip is exactly that box. */
    pub(crate) fn clip_box(self, s: &Computed, rect: [i32; 4], w: i32) -> Self {
        let mut c = self.clipped(rect);
        if c.clip == Some(rect) && s.clips_x() && s.clips_y() {
            let b = [s.border_top, s.border_right, s.border_bottom, s.border_left];
            let inner = |r: u16, i: usize| r.saturating_sub(b[i].max(b[(i + 3) % 4]) as u16);
            let r = radii(s, w);
            c.clip_r = [inner(r[0], 0), inner(r[1], 1), inner(r[2], 2), inner(r[3], 3)];
        }
        c
    }
}
