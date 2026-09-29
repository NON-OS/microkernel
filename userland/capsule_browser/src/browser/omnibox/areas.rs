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

use super::geometry::{BUBBLE_BAND, CONTENT_TOP};
use super::rect::Rect;

/* The page area below the toolbar, and the band at its bottom that holds
 * the hovered-link bubble. */
pub fn page_rect(width: u32, height: u32) -> Rect {
    Rect { x: 0, y: CONTENT_TOP, w: width, h: height.saturating_sub(CONTENT_TOP) }
}

pub fn bubble_band(width: u32, height: u32) -> Rect {
    let y = height.saturating_sub(BUBBLE_BAND).max(CONTENT_TOP);
    Rect { x: 0, y, w: width, h: height.saturating_sub(y) }
}
