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

use super::constants::TEXT_LEFT;
use super::draw_cursor::{draw_cursor, Caret};
use super::line_chars::chars_of;
use super::line_text::text_parts;
use super::line_window::{cells_of, window};
use super::metrics::Metrics;
use super::prompt::draw_prompt;
use super::suggestion::draw_suggestion;
use super::syntax::{classify, Part};

use crate::layout::Rect;
use crate::term::state::State;
use crate::term::theme::types::Theme;

#[path = "input_bar.rs"]
mod input_bar;
use input_bar::draw_bar;

pub fn draw_input_line(state: &State, fb: &mut PaintBuffer, r: Rect, m: Metrics, t: &Theme) {
    let adv = m.adv;
    let y = r.y;
    let ox = r.x + TEXT_LEFT;
    draw_bar(fb, r, m, t);
    // Character cells that fit between the left inset and an equal right margin.
    let total_cells = (r.w.saturating_sub(TEXT_LEFT * 2) / adv) as usize;
    // Prompt is user@host, path, mark and a space; cap it to half the line so
    // a deep cwd never starves the area left to type in.
    let prompt_cells = draw_prompt(state, fb, ox, y, m, total_cells / 2, t);
    // Horizontal scroll: slide a body_cells-wide window so the cursor is always
    // on screen, showing the start of the line whenever it fits.
    let body = state.line.as_bytes();
    let cursor = state.line.cursor.min(body.len());
    let body_cells = total_cells.saturating_sub(prompt_cells).max(1);
    let (start, stop, cursor_cell) = window(body, cursor, body_cells);
    let bx = ox + prompt_cells as u32 * adv;
    // Classify the whole line, not the visible window, so a word keeps its
    // colour when it scrolls in from either side.
    let mut parts = [Part::Plain; MAX_HIGHLIGHT];
    classify(body, &mut parts);
    let from = start.min(MAX_HIGHLIGHT);
    let to = stop.min(MAX_HIGHLIGHT);
    text_parts(fb, bx, y, &body[start..stop], &parts[from..to], m, t);
    // The offer sits where the cursor is, so it has to be drawn before the
    // cursor goes on top of it.
    let typed_cells = cells_of(&body[start..stop]);
    let ghost_x = bx + typed_cells as u32 * adv;
    let room = body_cells.saturating_sub(typed_cells);
    draw_suggestion(state, fb, ghost_x, y, m, room, t);
    let under = chars_of(&body[cursor..]).next();
    let caret = Caret { prompt_cells, cursor_cell, under };
    draw_cursor(fb, ox, &caret, y + 1, m, t);
}

// The longest line that is coloured. Past it the tail is drawn plain rather
// than growing a buffer on every keystroke; a command that long is being
// pasted, not read.
const MAX_HIGHLIGHT: usize = 1024;
