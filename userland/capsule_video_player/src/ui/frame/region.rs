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

use nonos_toolkit::widgets::Rect;

use super::geom::sidebar_w;
use crate::ui::layout::TOP;

pub const TOPBAR_H: u32 = 58;
pub const GUTTER: u32 = 20;
pub const VIEW_W: u32 = 78;
pub const HALF: u32 = VIEW_W / 2;

pub fn content(w: u32, h: u32) -> Rect {
    let x = sidebar_w(w);
    Rect { x, y: TOP, w: w.saturating_sub(x), h: h.saturating_sub(TOP) }
}

pub fn topbar(w: u32, h: u32) -> Rect {
    let c = content(w, h);
    Rect { x: c.x + GUTTER, y: c.y + 14, w: c.w.saturating_sub(GUTTER * 2), h: TOPBAR_H }
}

pub fn body(w: u32, h: u32) -> Rect {
    let c = content(w, h);
    let top = c.y + 14 + TOPBAR_H + 6;
    Rect {
        x: c.x + GUTTER,
        y: top,
        w: c.w.saturating_sub(GUTTER * 2),
        h: (c.y + c.h).saturating_sub(top).saturating_sub(GUTTER),
    }
}

pub fn search(w: u32, h: u32) -> Rect {
    let t = topbar(w, h);
    // Room is left on the right for the grid and list switch.
    let width = t.w.saturating_sub(140).clamp(180, 460);
    Rect { x: t.x, y: t.y + 8, w: width, h: 38 }
}

pub fn tool(w: u32, h: u32, offset: u32, width: u32) -> Rect {
    let t = topbar(w, h);
    let right = t.x + t.w;
    Rect { x: right.saturating_sub(offset + width), y: t.y + 8, w: width, h: 38 }
}

/// The grid and list switch at the right of the top bar.
pub fn view_toggle(w: u32, h: u32) -> Rect {
    tool(w, h, 0, VIEW_W)
}

pub fn grid_half(r: Rect) -> Rect {
    Rect { x: r.x, y: r.y, w: HALF, h: r.h }
}

pub fn list_half(r: Rect) -> Rect {
    Rect { x: r.x + HALF, y: r.y, w: r.w.saturating_sub(HALF), h: r.h }
}
