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

use super::layout::px;
use super::preview::Preview;
use super::preview_clip::clip;
use super::preview_info::info;

const LEFT: u32 = 16;
const FIRST_Y: u32 = 72;
const LINE_H: u32 = 14;
pub const MAX_COLS: usize = 38;
pub const VISIBLE_LINES: usize = 10;

// Render an opened file: title, path, a byte/line info bar, then the visible
// window of content lines starting at the preview's scroll offset.
pub fn paint_preview(preview: &Preview, fb: &mut PaintBuffer) {
    let (r, left, glyphs) = (roles(), px(LEFT), px(1));
    fb.clear(r.bg.argb());
    fb.text_scaled(left, px(18), b"file_manager", r.text.argb(), glyphs);
    fb.text_scaled(left, px(40), clip(preview.path.as_bytes()), r.accent.argb(), glyphs);
    fb.text_scaled(left, px(56), info(preview).as_bytes(), r.muted.argb(), glyphs);
    let mut y = px(FIRST_Y);
    for line in preview.lines.iter().skip(preview.scroll).take(VISIBLE_LINES) {
        fb.text_scaled(left, y, clip(line.as_bytes()), r.text.argb(), glyphs);
        y += px(LINE_H);
    }
    fb.text_scaled(left, px(232), b"esc back  up/down scroll", r.quiet.argb(), glyphs);
}
