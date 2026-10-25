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

//! Setting the cursor's edges from a display size, and moving them when the
//! size changes (cursor_edges.rs says when).

use super::CursorState;

impl CursorState {
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
        (self.max_x, self.max_y) = (max_x, max_y);
        (self.frac_x, self.frac_y) = (0, 0);
        self.clamp();
    }
}

fn rescale(v: i32, old_max: i32, new_max: i32) -> i32 {
    if old_max <= 0 {
        return new_max / 2;
    }
    (i64::from(v) * i64::from(new_max) / i64::from(old_max)) as i32
}
