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

//! The rectangles inside the frame: its title bar and the app's content.

use super::frame_rect::{border_at, frame_rect_at, titlebar_h_at};
use super::rect::Rect;
use super::scale::ONE;

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

/// The extra width and height the frame takes at `quarters` over one to one,
/// for an unmaximised window: what a window grows by to keep the content area
/// its app asked for.
pub fn chrome_growth_at(quarters: u32) -> (u32, u32) {
    let at1 = content_rect_at(1000, 1000, false, ONE);
    let atq = content_rect_at(1000, 1000, false, quarters);
    (at1.w - atq.w, at1.h - atq.h)
}
