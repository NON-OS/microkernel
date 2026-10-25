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

//! A toast row's measures, in logical pixels scaled to the screen: its
//! padding, the gap between rows, the level's accent, the text's inset, and
//! from them a row's and the whole panel's height.

use crate::render::ui_font::{line_h, px, UI_PX};
use crate::state::toasts::MAX_TOASTS;

const ROW_PAD_LOGICAL: u32 = 12;
const ROW_GAP_LOGICAL: u32 = 2;
const ACCENT_WIDTH_LOGICAL: u32 = 4;
const TEXT_INSET_LOGICAL: u32 = 12;

pub(super) fn row_pad() -> u32 {
    px(ROW_PAD_LOGICAL)
}

pub(super) fn row_gap() -> u32 {
    px(ROW_GAP_LOGICAL)
}

pub(super) fn accent_width() -> u32 {
    px(ACCENT_WIDTH_LOGICAL)
}

pub(super) fn text_inset() -> u32 {
    px(TEXT_INSET_LOGICAL)
}

pub(super) fn panel_edge_inset() -> u32 {
    px(2)
}

pub(super) fn row_height() -> u32 {
    line_h(UI_PX) + row_pad()
}

pub(super) fn panel_height() -> u32 {
    row_gap() + MAX_TOASTS as u32 * (row_height() + row_gap())
}

pub(super) fn row_chrome_width() -> u32 {
    panel_edge_inset() * 2 + accent_width() + text_inset() * 2
}
