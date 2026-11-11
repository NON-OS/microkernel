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

use super::layout::{font, px};
use super::sidebar_metrics::{line_box, rail};

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
    let (r, rl) = (roles(), rail());
    let (side, top, mark_px, word_px) = (px(MARK_S), px(MARK_TOP), font(MARK_PX), font(WORD_PX));
    Card::new().draw(fb, Rect { x: rl.x, y: top, w: side, h: side });
    let gx = rl.x + side.saturating_sub(text::width(MARK, mark_px)) / 2;
    let gy = top + side.saturating_sub(line_box(mark_px)) / 2;
    text::left(fb, gx, gy as i32, MARK, r.accent, mark_px);
    let tx = rl.text_x;
    let avail = (rl.x + rl.w).saturating_sub(tx);
    let word = text::truncate(WORD, avail, word_px);
    let wy = top + side.saturating_sub(line_box(word_px)) / 2;
    text::left(fb, tx, wy as i32, &word, r.text, word_px);
}
