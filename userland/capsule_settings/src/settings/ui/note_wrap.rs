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
//! A card's note, broken into lines that fit the card. The walker sizes the
//! head by the same lines the painter draws, so a long note grows its card
//! rather than running past the edge.

use nonos_toolkit::font::ttf::{line_height, measure};

use super::metrics::{CARD_HEAD_NOTE_H, CARD_PAD_X, NOTE_PX, PANE_PAD_X, SIDEBAR_W};

/// The width a note line may take in a window `win_w` wide.
pub fn note_w(win_w: u32) -> u32 {
    win_w.saturating_sub(SIDEBAR_W + PANE_PAD_X * 2 + CARD_PAD_X * 2)
}

/// Calls `f` with each line of `note`, as many words as fit `max_w`, and
/// says how many lines there were. A word wider than `max_w` takes a line
/// of its own.
pub fn lines<'a>(note: &'a str, max_w: u32, mut f: impl FnMut(&'a str)) -> u32 {
    let fits = |s: &str| measure(s, NOTE_PX, false).max(0) as u32 <= max_w;
    let mut rest = note.trim();
    let mut n = 0;
    while !rest.is_empty() {
        let mut end = rest.len();
        if !fits(rest) {
            let first = rest.find(' ').unwrap_or(rest.len());
            let spaces = rest.match_indices(' ').map(|(i, _)| i);
            end = spaces.take_while(|&i| fits(&rest[..i])).last().unwrap_or(first);
        }
        f(&rest[..end]);
        n += 1;
        rest = rest[end..].trim_start();
    }
    n
}

/// The height of a card head whose note is `note`, in a window `win_w` wide.
pub fn head_h(note: &str, win_w: u32) -> u32 {
    let n = lines(note, note_w(win_w), |_| {});
    CARD_HEAD_NOTE_H + n.saturating_sub(1) * line_height(NOTE_PX).max(0) as u32
}
