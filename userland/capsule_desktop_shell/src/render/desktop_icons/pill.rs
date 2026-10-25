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

//! The rounded pill a desktop entry's name sits on: its padding, its height
//! from the label's line, where the text goes inside it, and the pill drawn.

use crate::render::layout::Rect;
use crate::render::palette;
use crate::render::ui_font::{line_h, px, top_y_centered, LABEL_PX};
use crate::state::Context;

const PILL_PAD_LOGICAL: u32 = 6;
const PILL_VPAD_LOGICAL: u32 = 2;

pub(super) fn pill_pad() -> u32 {
    px(PILL_PAD_LOGICAL)
}

pub(super) fn pill_vpad() -> u32 {
    px(PILL_VPAD_LOGICAL)
}

pub(super) fn label_h() -> u32 {
    line_h(LABEL_PX)
}

pub(super) fn pill_h() -> u32 {
    label_h() + pill_vpad() * 2
}

pub(super) fn text_top(label_y: u32) -> u32 {
    top_y_centered(label_y.saturating_sub(pill_vpad()), pill_h(), LABEL_PX)
}

pub(super) fn pill(ctx: &Context, pill_x: u32, label_y: u32, text_w: u32, bg: u32) {
    let rect = Rect {
        x: pill_x,
        y: label_y.saturating_sub(pill_vpad()),
        width: text_w + pill_pad() * 2,
        height: pill_h(),
    };
    crate::render::panel::round_fill(ctx, rect, palette::R_TILE, bg);
}
