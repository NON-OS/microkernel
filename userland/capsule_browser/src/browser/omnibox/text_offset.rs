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

/* Room kept to the right of the caret when the text scrolls. */
const MARGIN: i32 = 4;

/* Horizontal scroll, in pixels, of a one-line field `field_w` wide that
 * holds `text_px` of text, so the caret at `caret_px` stays in view. It
 * keeps `prev` while the caret is visible, which stops the text jumping
 * on every key, and never scrolls past what the text needs. */
pub fn text_offset(caret_px: i32, text_px: i32, field_w: i32, prev: i32) -> i32 {
    if text_px + MARGIN <= field_w {
        return 0;
    }
    let max = text_px + MARGIN - field_w;
    let mut off = prev.clamp(0, max);
    if caret_px - off > field_w - MARGIN {
        off = caret_px - field_w + MARGIN;
    }
    if caret_px < off {
        off = caret_px;
    }
    off.clamp(0, max)
}
