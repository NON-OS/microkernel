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

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Card, Rect};

use super::sidebar_metrics::{line_box, rail_text_x, rail_w, RAIL_X};

const MARK_S: u32 = 34;
const MARK_TOP: u32 = 26;
const MARK: &str = "Ø";
const MARK_PX: f32 = 20.0;
const WORD: &str = "NØNOS Files";
const WORD_PX: f32 = 20.0;

/// The rail's masthead: the mark on its own card with the wordmark set beside
/// it. Both runs are placed by measurement, so the glyph centres in the card
/// and the wordmark cuts at the rail edge rather than at a glyph count.
pub fn paint_brand(fb: &mut PaintBuffer) {
    let r = roles();
    Card::new().draw(fb, Rect { x: RAIL_X, y: MARK_TOP, w: MARK_S, h: MARK_S });
    let gx = RAIL_X + MARK_S.saturating_sub(text::width(MARK, MARK_PX)) / 2;
    let gy = MARK_TOP + MARK_S.saturating_sub(line_box(MARK_PX)) / 2;
    text::left(fb, gx, gy as i32, MARK, r.accent, MARK_PX);
    let tx = rail_text_x();
    let avail = (RAIL_X + rail_w()).saturating_sub(tx);
    let word = text::truncate(WORD, avail, WORD_PX);
    let wy = MARK_TOP + MARK_S.saturating_sub(line_box(WORD_PX)) / 2;
    text::left(fb, tx, wy as i32, &word, r.text, WORD_PX);
}
