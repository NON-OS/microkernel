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
use nonos_toolkit::widgets::rect::{centre, inset};
use nonos_toolkit::widgets::Rect;

use super::metrics::metrics;
use super::rect::{self, at};

pub const ACTIONS: usize = 4;

pub const LABELS: [&[u8]; ACTIONS] = [b"Resume", b"Restart", b"Settings", b"Home"];

pub fn title_h() -> u32 {
    line_height(metrics().px_title).max(1) as u32
}

// The modal floats over the live board, so it is centred on the whole surface
// rather than on the content rect the other screens share. Its height follows
// the title and the action stack, which is all it ever holds.
pub fn modal(w: u32, h: u32) -> Rect {
    let m = metrics();
    let span = m.btn_h * ACTIONS as u32 + m.btn_gap * (ACTIONS as u32 - 1);
    centre(at(0, 0, w, h), m.modal_w, m.pad * 2 + title_h() + m.gap_wide + span)
}

pub fn title(w: u32, h: u32) -> Rect {
    let inner = inset(modal(w, h), metrics().pad);
    Rect { h: title_h(), ..inner }
}

pub fn action(w: u32, h: u32, index: usize) -> Rect {
    let m = metrics();
    let inner = inset(modal(w, h), m.pad);
    let span = m.btn_h * ACTIONS as u32 + m.btn_gap * (ACTIONS as u32 - 1);
    let top = inner.y + inner.h.saturating_sub(span);
    rect::row(at(inner.x, top, inner.w, span), index, m.btn_h, m.btn_gap)
}

pub fn action_at(w: u32, h: u32, x: i32, y: i32) -> Option<usize> {
    rect::index_at(ACTIONS, x, y, |i| action(w, h, i))
}
