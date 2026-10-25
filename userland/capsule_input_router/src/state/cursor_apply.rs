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

//! Moving the cursor by one pointer event: relative motion at the sensitivity,
//! an absolute position or touch placed outright on the edges.

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH};

use super::edges::place;
use super::CursorState;

impl CursorState {
    pub fn apply(&mut self, ev: &InputEvent) -> (u32, u32) {
        if ev.kind == INPUT_KIND_POINTER_REL {
            let sx = ev.delta_x.saturating_mul(self.mult_x2).saturating_add(self.frac_x);
            let sy = ev.delta_y.saturating_mul(self.mult_x2).saturating_add(self.frac_y);
            self.x = self.x.saturating_add(sx / 2);
            self.y = self.y.saturating_add(sy / 2);
            self.frac_x = sx % 2;
            self.frac_y = sy % 2;
        }
        if ev.kind == INPUT_KIND_POINTER_ABS || ev.kind == INPUT_KIND_TOUCH {
            // Absolute devices place the cursor outright; drop any carried
            // remainder so it does not skew the next relative motion.
            (self.x, self.y) = (place(ev.x, self.max_x), place(ev.y, self.max_y));
            self.frac_x = 0;
            self.frac_y = 0;
        }
        self.clamp();
        (self.x as u32, self.y as u32)
    }
}
