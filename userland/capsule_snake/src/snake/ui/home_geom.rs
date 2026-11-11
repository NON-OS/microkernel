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

use nonos_toolkit::font::ttf::line_height;
use nonos_toolkit::widgets::rect::centre;
use nonos_toolkit::widgets::Rect;

use super::metrics::metrics;
use super::rect::{self, at};

pub const ACTIONS: usize = 4;
pub const CARDS: usize = 2;

pub const LABELS: [&[u8]; ACTIONS] = [b"Play", b"Continue", b"Ranks", b"Settings"];
pub const CARD_LABELS: [&[u8]; CARDS] = [b"Daily challenge", b"Best run"];

pub fn wordmark_h() -> u32 {
    let m = metrics();
    line_height(m.px_wordmark).max(1) as u32 + line_height(m.px_body).max(1) as u32
}

fn stack_h() -> u32 {
    let m = metrics();
    wordmark_h() + m.gap_wide + m.btn_h * 2 + m.btn_gap + m.gap_wide + m.hud_h
}

// The hub is one centred column: wordmark, a primary Play, a row of three, then
// the two summary cards. Everything below reads its origin from here.
pub fn stack(w: u32, h: u32) -> Rect {
    let m = metrics();
    centre(rect::content(w, h), m.btn_w * 2 + m.gap, stack_h())
}

pub fn wordmark(w: u32, h: u32) -> Rect {
    let s = stack(w, h);
    Rect { h: wordmark_h(), ..s }
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let (s, m) = (stack(w, h), metrics());
    let top = s.y + wordmark_h() + m.gap_wide;
    if index == 0 {
        return at(s.x, top, s.w, m.btn_h);
    }
    let band = at(s.x, top + m.btn_h + m.btn_gap, s.w, m.btn_h);
    rect::column(band, index - 1, ACTIONS - 1, m.btn_gap)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}

pub fn card(w: u32, h: u32, index: usize) -> Rect {
    let (s, m) = (stack(w, h), metrics());
    let band = at(s.x, s.y + s.h.saturating_sub(m.hud_h), s.w, m.hud_h);
    rect::column(band, index, CARDS, m.gap)
}

pub fn card_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(CARDS, x, y, |i| card(w, h, i))
}
