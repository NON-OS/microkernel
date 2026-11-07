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

//! A passphrase field: a box with a dot for each character typed, never
//! the characters, or a hint while it is empty.

use super::super::dialog_kit::{border, fill};
use super::super::dialog_text::line;
use crate::render::palette;
use crate::render::ui_font::{px, top_y_centered, UI_PX};
use crate::state::Context;

const FIELD_BG: u32 = 0xFF0E_1620;
const DOT: u32 = 0xFFDF_EAF7;
const HINT: u32 = 0xFF7A_8D99;

pub(super) fn paint(ctx: &Context, r: (u32, u32, u32, u32), chars: usize, hint: &str, live: bool) {
    let b = super::rect(r);
    fill(ctx, b, FIELD_BG);
    border(ctx, b, if live { palette::ACCENT } else { palette::LINE_HARD });
    let pad = px(14);
    if chars == 0 {
        let y = top_y_centered(b.y, b.height, UI_PX);
        line(ctx, b.x + pad, y, hint, HINT, UI_PX, b.width.saturating_sub(2 * pad));
        return;
    }
    let (d, step) = (px(8), px(14));
    let room = (b.width.saturating_sub(2 * pad) / step) as usize;
    let y = b.y + b.height.saturating_sub(d) / 2;
    for i in 0..chars.min(room) {
        let x = b.x + pad + i as u32 * step;
        fill(ctx, crate::render::layout::Rect { x, y, width: d, height: d }, DOT);
    }
}
