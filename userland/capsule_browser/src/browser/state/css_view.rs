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

use super::State;
use crate::browser::layout::boxmodel::css_px;

impl State {
    /// The page area in CSS px: its drawing px over the display scale, the
    /// viewport the page's styles, media queries, layout and scripts see.
    pub fn css_viewport(&self) -> (u32, u32) {
        let css = |v: u32| css_px(v.min(i32::MAX as u32) as i32).max(0) as u32;
        (css(self.viewport_w), css(self.viewport_h))
    }
}
