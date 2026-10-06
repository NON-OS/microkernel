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

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS, INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH};

const ABS_RANGE_MAX: i64 = 0x7FFF;

pub struct CursorState {
    pub x: i32,
    pub y: i32,
    max_x: i32,
    max_y: i32,
    pub configured: bool,
    pub mult_x2: i32,
    // Sub-pixel remainder from the fixed-point scale, carried into the next
    // event so slow motion at a fractional sensitivity is not truncated away.
    frac_x: i32,
    frac_y: i32,
}

impl CursorState {
    pub const fn new() -> Self {
        Self {
            x: 512,
            y: 384,
            max_x: 1023,
            max_y: 767,
            configured: false,
            mult_x2: 2,
            frac_x: 0,
            frac_y: 0,
        }
    }

    pub fn configure(&mut self, width: u32, height: u32) {
        self.max_x = width.saturating_sub(1).min(i32::MAX as u32) as i32;
        self.max_y = height.saturating_sub(1).min(i32::MAX as u32) as i32;
        self.x = self.max_x / 2;
        self.y = self.max_y / 2;
        self.configured = true;
    }

    // The display is now `width` by `height`. The cursor keeps its place in
    // proportion, so it stays over what it was over when the picture is
    // scaled, and stays inside a display that shrank.
    pub fn resize(&mut self, width: u32, height: u32) {
        let max_x = width.saturating_sub(1).min(i32::MAX as u32) as i32;
        let max_y = height.saturating_sub(1).min(i32::MAX as u32) as i32;
        if (max_x, max_y) == (self.max_x, self.max_y) {
            return;
        }
        self.x = rescale(self.x, self.max_x, max_x);
        self.y = rescale(self.y, self.max_y, max_y);
        self.max_x = max_x;
        self.max_y = max_y;
        self.frac_x = 0;
        self.frac_y = 0;
        self.clamp();
    }

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
            self.x = (ev.x as i64 * self.max_x as i64 / ABS_RANGE_MAX) as i32;
            self.y = (ev.y as i64 * self.max_y as i64 / ABS_RANGE_MAX) as i32;
            self.frac_x = 0;
            self.frac_y = 0;
        }
        self.clamp();
        (self.x as u32, self.y as u32)
    }

    fn clamp(&mut self) {
        self.x = self.x.clamp(0, self.max_x);
        self.y = self.y.clamp(0, self.max_y);
    }
}

fn rescale(v: i32, old_max: i32, new_max: i32) -> i32 {
    if old_max <= 0 {
        return new_max / 2;
    }
    (i64::from(v) * i64::from(new_max) / i64::from(old_max)) as i32
}

impl Default for CursorState {
    fn default() -> Self {
        Self::new()
    }
}
