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

//! A desktop entry's name while it is being renamed: the text typed so far
//! on an edit pill with a caret after it, its tail kept in view when it
//! outgrows the cell.

use super::metrics::caret_w;
use super::pill::{label_h, pill, pill_pad, text_top};
use crate::render::layout::Rect;
use crate::render::measure_aa::measure_aa;
use crate::render::palette;
use crate::render::text_aa::text_aa;
use crate::render::ui_font::LABEL_PX;
use crate::state::Context;

const EDIT_BG: u32 = palette::PILL_EDIT;
const EDIT_FG: u32 = 0xFFFF_FFFF;
const CARET: u32 = palette::ACCENT;

// The inline rename field: the edit buffer with a caret, on a lit pill.
pub(super) fn paint_rename(ctx: &Context, cx: u32, cw: u32, label_y: u32) {
    let room = cw.saturating_sub(pill_pad() * 2 + caret_w());
    let shown = tail_to_width(&ctx.desk_ui.rename_buf, room);
    let text_w = measure_aa(shown, LABEL_PX);
    let pill_x = cx + cw.saturating_sub(text_w + pill_pad() * 2 + caret_w()) / 2;
    pill(ctx, pill_x, label_y, text_w + caret_w(), EDIT_BG);
    text_aa(ctx, pill_x + pill_pad(), text_top(label_y), shown, EDIT_FG, LABEL_PX);
    let caret_x = pill_x + pill_pad() + text_w;
    let caret = Rect { x: caret_x, y: label_y, width: caret_w(), height: label_h() };
    crate::render::panel::blend(ctx, caret, CARET);
}

// The longest suffix that fits, so the caret end of a long edit stays on screen.
pub(super) fn tail_to_width(text: &str, max_w: u32) -> &str {
    let mut start = text.len();
    for (i, _) in text.char_indices().rev() {
        if measure_aa(&text[i..], LABEL_PX) > max_w {
            break;
        }
        start = i;
    }
    &text[start..]
}
