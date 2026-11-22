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

//! The window's size as a script reads it: `innerWidth`, `innerHeight`,
//! `devicePixelRatio` and `screen`. None of them were defined, so a page
//! sizing a canvas or choosing a layout from them read undefined.

use super::tree::Dom;

impl Dom {
    /// One of the window's measures for the bindings, by `which`: 0 the
    /// viewport's width and 1 its height, in CSS px; 2 the display scale
    /// `q` in quarters, which the page reads as `devicePixelRatio` q/4.
    /// Anything else is 0.
    ///
    /// There is no screen apart from the window here, so `screen` reads
    /// the viewport too.
    pub fn window_metric(&self, which: i32, q: u32) -> i32 {
        let v = match which {
            0 => self.viewport.0,
            1 => self.viewport.1,
            2 => q,
            _ => 0,
        };
        v.min(i32::MAX as u32) as i32
    }
}
