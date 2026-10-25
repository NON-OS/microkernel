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

use nonos_libc::INPUT_ABS_MAX;

#[path = "cursor_apply.rs"]
mod apply;
#[path = "cursor_edges.rs"]
mod edges;
#[path = "cursor_resize.rs"]
mod resize;

// Every absolute event spans the whole display as 0 to INPUT_ABS_MAX on both
// axes, whatever the device reported; the drivers scale into it.
const ABS_RANGE_MAX: i64 = INPUT_ABS_MAX as i64;

pub struct CursorState {
    pub x: i32,
    pub y: i32,
    max_x: i32,
    max_y: i32,
    /// The edges are the display's, as the compositor said (cursor_edges.rs).
    pub configured: bool,
    pub mult_x2: i32,
    // Sub-pixel remainder from the fixed-point scale, carried into the next
    // event so slow motion at a fractional sensitivity is not truncated away.
    frac_x: i32,
    frac_y: i32,
    // Where an absolute device pointed before the edges were known.
    held: Option<(i32, i32)>,
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
            held: None,
        }
    }

    fn clamp(&mut self) {
        self.x = self.x.clamp(0, self.max_x);
        self.y = self.y.clamp(0, self.max_y);
    }
}

impl Default for CursorState {
    fn default() -> Self {
        Self::new()
    }
}
