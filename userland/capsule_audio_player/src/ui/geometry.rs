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

use super::metrics::{line_h, metrics, px};

/// The shared widgets' box, which the player's signed boxes are drawn in.
pub use nonos_toolkit::widgets::Rect as Drawn;

pub const SIDEBAR_W: i32 = 236;
pub const RAIL_W: i32 = 300;
pub const TOPBAR_H: i32 = 58;
pub const TRANSPORT_H: i32 = 74;
/// The narrowest window that keeps the right rail, logical like the rest.
pub const RAIL_FROM: i32 = 1080;

#[derive(Clone, Copy)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new(x: i32, y: i32, w: i32, h: i32) -> Rect {
        Rect { x, y, w: w.max(0), h: h.max(0) }
    }

    pub fn contains(&self, px: i32, py: i32) -> bool {
        px >= self.x && px < self.x + self.w && py >= self.y && py < self.y + self.h
    }

    pub fn right(&self) -> i32 {
        self.x + self.w
    }

    pub fn bottom(&self) -> i32 {
        self.y + self.h
    }

    pub fn cx(&self) -> i32 {
        self.x + self.w / 2
    }

    pub fn cy(&self) -> i32 {
        self.y + self.h / 2
    }

    pub fn inset(&self, d: i32) -> Rect {
        Rect::new(self.x + d, self.y + d, self.w - d * 2, self.h - d * 2)
    }

    pub fn pad(&self, dx: i32, dy: i32) -> Rect {
        Rect::new(self.x + dx, self.y + dy, self.w - dx * 2, self.h - dy * 2)
    }

    pub fn row(&self, y: i32, h: i32) -> Rect {
        Rect::new(self.x, self.y + y, self.w, h)
    }

    pub fn centred(&self, w: i32, h: i32) -> Rect {
        Rect::new(self.x + (self.w - w) / 2, self.y + (self.h - h) / 2, w, h)
    }

    /// This box on the shared widgets' unsigned grid, cut where it runs past
    /// the window's left or top edge.
    pub fn tk(&self) -> Drawn {
        let (x, y) = (self.x.max(0), self.y.max(0));
        let (w, h) = ((self.right() - x).max(0), (self.bottom() - y).max(0));
        Drawn { x: x as u32, y: y as u32, w: w as u32, h: h as u32 }
    }
}

/// Where `x` falls along `r`, in thousandths clamped to 0..=1000: a click on
/// a groove asks for the value drawn under it.
pub fn permille(r: Rect, x: i32) -> u32 {
    if r.w <= 0 {
        return 0;
    }
    (((x - r.x) as i64 * 1000 / r.w as i64).clamp(0, 1000)) as u32
}

pub struct Shell {
    pub frame: Rect,
    pub sidebar: Rect,
    pub topbar: Rect,
    pub content: Rect,
    pub rail: Rect,
    pub transport: Rect,
}

pub fn shell(width: u32, height: u32) -> Shell {
    let (w, h, s6) = (width as i32, height as i32, metrics().s6);
    let (side, top, foot) = (px(SIDEBAR_W), px(TOPBAR_H), px(TRANSPORT_H));
    let tr_y = h - foot;
    let rail_w = if w >= px(RAIL_FROM) { px(RAIL_W) } else { 0 };
    let main_x = side;
    let main_w = w - side - rail_w;
    Shell {
        frame: Rect::new(0, 0, w, h),
        sidebar: Rect::new(0, 0, side, tr_y),
        topbar: Rect::new(main_x, 0, main_w, top),
        content: Rect::new(main_x + s6, top, main_w - s6 * 2, tr_y - top),
        rail: Rect::new(main_x + main_w, 0, rail_w, tr_y),
        transport: Rect::new(0, tr_y, w, foot),
    }
}

pub fn page(sh: &Shell) -> Rect {
    sh.content.pad(0, metrics().s6)
}

/// The band at the top of a page that its title and the line under it take.
pub fn page_head(r: Rect) -> Rect {
    let m = metrics();
    Rect::new(r.x, r.y, r.w, line_h(m.page) + line_h(m.body))
}
