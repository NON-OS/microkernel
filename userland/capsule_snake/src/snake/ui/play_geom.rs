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

use crate::snake::grid::{COLS, ROWS};

use super::metrics::{FOOT_H, GAP, HUD_H, RAIL_W};
use super::rect::{self, at};

pub struct Board {
    pub cell: u32,
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Board {
    pub fn inset(&self) -> u32 {
        (self.cell / 16).max(1)
    }
}

pub fn hud_band(w: u32, h: u32) -> Rect {
    Rect { h: HUD_H, ..rect::content(w, h) }
}

pub fn foot_band(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    at(c.x, c.y + c.h.saturating_sub(FOOT_H), c.w, FOOT_H)
}

fn middle(w: u32, h: u32) -> Rect {
    let c = rect::content(w, h);
    let top = c.y + HUD_H + GAP;
    let h = c.h.saturating_sub(HUD_H + FOOT_H + GAP * 2);
    at(c.x, top, c.w, h)
}

pub fn rail(w: u32, h: u32) -> Rect {
    let m = middle(w, h);
    let rail_w = RAIL_W.min(m.w);
    at(m.x + m.w.saturating_sub(rail_w), m.y, rail_w, m.h)
}

pub fn stage(w: u32, h: u32) -> Rect {
    let m = middle(w, h);
    Rect { w: m.w.saturating_sub(RAIL_W.min(m.w) + GAP), ..m }
}

// The cell size is re-derived from the live surface every call, never from the
// nominal CELL, which is what keeps maximize working.
pub fn board(w: u32, h: u32) -> Board {
    let s = stage(w, h);
    let cell = (s.w / COLS as u32).min(s.h / ROWS as u32).max(4);
    let bw = cell * COLS as u32;
    let bh = cell * ROWS as u32;
    let x = s.x + s.w.saturating_sub(bw) / 2;
    let y = s.y + s.h.saturating_sub(bh) / 2;
    Board { cell, x, y, w: bw, h: bh }
}
