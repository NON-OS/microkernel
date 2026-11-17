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

use super::metrics::metrics;
use super::play_geom::{foot_band, hud_band, rail};
use super::rect::{self, at};

pub const HUD_CARDS: usize = 4;
pub const FOOT_BTNS: usize = 3;
pub const RAIL_ROWS: usize = 4;

pub const FOOT_LABELS: [&[u8]; FOOT_BTNS] = [b"Pause", b"Restart", b"Home"];
pub const RAIL_HEADS: [&[u8]; RAIL_ROWS] = [b"Mode", b"Level", b"Next level", b"Tip"];

pub fn hud(w: u32, h: u32, index: usize) -> Rect {
    rect::column(hud_band(w, h), index, HUD_CARDS, metrics().hud_card_gap)
}

// The footer row is right-aligned so the primary action sits under the rail
// rather than drifting with the board width.
pub fn foot(w: u32, h: u32, index: usize) -> Rect {
    let (band, m) = (foot_band(w, h), metrics());
    let span = m.foot_btn_w * FOOT_BTNS as u32 + m.gap * (FOOT_BTNS as u32 - 1);
    let x = band.x + band.w.saturating_sub(span) + index as u32 * (m.foot_btn_w + m.gap);
    let y = band.y + m.foot_h.saturating_sub(m.foot_btn_h) / 2;
    at(x, y, m.foot_btn_w, m.foot_btn_h)
}

pub fn foot_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(FOOT_BTNS, x, y, |i| foot(w, h, i))
}

/// The rail less `PAD_TIGHT` top and bottom, at its full width, since the
/// rows drawn in it pad their own sides.
pub fn rail_inner(w: u32, h: u32) -> Rect {
    let (r, pad) = (rail(w, h), metrics().pad_tight);
    Rect { y: r.y + pad, h: r.h.saturating_sub(pad * 2), ..r }
}

pub fn rail_row(w: u32, h: u32, index: usize) -> Rect {
    let m = metrics();
    rect::row(rail_inner(w, h), index, m.row_h, m.gap_tight)
}
