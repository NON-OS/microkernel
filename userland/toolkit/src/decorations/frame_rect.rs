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

use super::metrics::{
    BORDER_PX, FRAME_RADIUS, LIGHT_D, LIGHT_GAP, LIGHT_INSET, SHADOW_MARGIN, TITLEBAR_H,
};
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

pub fn titlebar_rect(w: u32, h: u32, maximized: bool) -> Rect {
    titlebar_rect_at(w, h, maximized, ONE)
}

pub fn titlebar_rect_at(w: u32, h: u32, maximized: bool, quarters: u32) -> Rect {
    let f = frame_rect_at(w, h, maximized, quarters);
    Rect { x: f.x, y: f.y, w: f.w, h: f.h.min(titlebar_h_at(quarters)) }
}

pub fn content_rect(w: u32, h: u32, maximized: bool) -> Rect {
    content_rect_at(w, h, maximized, ONE)
}

pub fn content_rect_at(w: u32, h: u32, maximized: bool, quarters: u32) -> Rect {
    let f = frame_rect_at(w, h, maximized, quarters);
    let (bar, border) = (titlebar_h_at(quarters), border_at(quarters));
    Rect {
        x: f.x + border,
        y: f.y + bar,
        w: f.w.saturating_sub(border * 2),
        h: f.h.saturating_sub(bar + border),
    }
}

pub fn light_rect(i: u32, w: u32, h: u32, maximized: bool) -> Rect {
    light_rect_at(i, w, h, maximized, ONE)
}

pub fn light_rect_at(i: u32, w: u32, h: u32, maximized: bool, quarters: u32) -> Rect {
    let f = frame_rect_at(w, h, maximized, quarters);
    let d = at(LIGHT_D, quarters);
    // The step between buttons is scaled as one length, so the third button
    // lands where three scaled steps put it rather than drifting by rounding.
    let step = at(LIGHT_D + LIGHT_GAP, quarters);
    Rect {
        x: f.x + at(LIGHT_INSET, quarters) + i * step,
        y: f.y + titlebar_h_at(quarters).saturating_sub(d) / 2,
        w: d,
        h: d,
    }
}

/// The extra width and height the frame takes at `quarters` over one to one,
/// for an unmaximised window: what a window grows by to keep the content area
/// its app asked for.
pub fn chrome_growth_at(quarters: u32) -> (u32, u32) {
    let at1 = content_rect_at(1000, 1000, false, ONE);
    let atq = content_rect_at(1000, 1000, false, quarters);
    (at1.w - atq.w, at1.h - atq.h)
}
