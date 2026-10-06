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

//! Which slice of a long line is on screen.
//!
//! The line is UTF-8 and the screen is cells: a character takes one to four
//! bytes and none to two cells. Counting bytes as cells put the cursor block
//! to the right of where the caret was, by a cell for every extra byte
//! before it, and scrolled a line of accented text before it was full.

use nonos_vt::width::width;

use super::line_chars::chars_of;

/// The cells `bytes` take on screen.
pub fn cells_of(bytes: &[u8]) -> usize {
    chars_of(bytes).map(width).sum()
}

/// How many bytes of `bytes` fit in `cells`, whole characters only.
pub fn fit_cells(bytes: &[u8], cells: usize) -> usize {
    let (mut used, mut end) = (0usize, 0usize);
    for ch in chars_of(bytes) {
        let w = width(ch);
        if used + w > cells {
            break;
        }
        used += w;
        end += ch.len_utf8();
    }
    end
}

/// The byte range of the line to draw in `cells`, so the cursor is always
/// visible, and the cell the cursor is in from the window's left edge.
pub fn window(body: &[u8], cursor: usize, cells: usize) -> (usize, usize, usize) {
    let cursor = cursor.min(body.len());
    let cursor_col = cells_of(&body[..cursor]);
    // The cursor needs a cell of its own, the last one at the furthest.
    let scroll = (cursor_col + 1).saturating_sub(cells.max(1));
    // Start at the first character wholly right of the scroll: a wide
    // character cut by the left edge is left out rather than drawn in half.
    let (mut start, mut col) = (0usize, 0usize);
    for ch in chars_of(body) {
        if col >= scroll {
            break;
        }
        col += width(ch);
        start += ch.len_utf8();
    }
    let stop = start + fit_cells(&body[start..], cells);
    (start, stop.max(start), cursor_col.saturating_sub(col))
}
