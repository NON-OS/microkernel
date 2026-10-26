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

use super::metrics::{BORDER_PX, FRAME_RADIUS, SHADOW_MARGIN, TITLEBAR_H};
use super::rect::Rect;
use super::scale::{at, ONE};

/* Each measure has a form at one to one, which every caller outside the app
 * runtime uses, and an `_at` form for `quarters` of display scale. */

pub fn margin(maximized: bool) -> u32 {
    margin_at(maximized, ONE)
}

pub fn margin_at(maximized: bool, quarters: u32) -> u32 {
    if maximized {
        0
    } else {
        at(SHADOW_MARGIN, quarters)
    }
}

pub fn radius(maximized: bool) -> u32 {
    radius_at(maximized, ONE)
}

pub fn radius_at(maximized: bool, quarters: u32) -> u32 {
    if maximized {
        0
    } else {
        at(FRAME_RADIUS, quarters)
    }
}

/// The frame's border width.
pub fn border_at(quarters: u32) -> u32 {
    at(BORDER_PX, quarters)
}

/// The title bar's height.
pub fn titlebar_h_at(quarters: u32) -> u32 {
    at(TITLEBAR_H, quarters)
}

pub fn frame_rect(w: u32, h: u32, maximized: bool) -> Rect {
    frame_rect_at(w, h, maximized, ONE)
}

pub fn frame_rect_at(w: u32, h: u32, maximized: bool, quarters: u32) -> Rect {
    let m = margin_at(maximized, quarters);
    Rect { x: m, y: m, w: w.saturating_sub(m * 2), h: h.saturating_sub(m * 2) }
}
