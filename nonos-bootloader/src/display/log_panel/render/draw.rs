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

use super::clear::clear_line;
use crate::display::ink::palette::{ACCENT, BAD, OK, TEXT_2, TEXT_3, WARN};
use crate::display::ink::{dot, draw, unit, width, Style};
use crate::display::log_panel::buffer::get_entry;
use crate::display::log_panel::types::{get_log_area, line_clear_width, line_height, LogLevel};

/// One line of the verification card: a state dot and the text, cut at a
/// character boundary so it never runs past the card.
pub fn draw_entry_at(line_num: usize, entry_idx: usize) {
    let (log_x, log_y) = get_log_area();
    let y = log_y + (line_num as u32) * line_height();
    clear_line(line_num);
    let Some(entry) = get_entry(entry_idx) else { return };
    if entry.len == 0 {
        return;
    }
    let (mark, text) = match entry.level {
        LogLevel::Ok => (OK, TEXT_2),
        LogLevel::Info => (TEXT_3, TEXT_3),
        LogLevel::Warn => (WARN, WARN),
        LogLevel::Error => (BAD, BAD),
        LogLevel::Security => (ACCENT, ACCENT),
    };
    let u = unit();
    let d = dot(log_x, y, Style::Mono, mark);
    let tx = log_x + d + 2 * u;
    let room = line_clear_width().saturating_sub(d + 2 * u);
    let mut n = entry.len.min(entry.text.len());
    while n > 0 && width(&entry.text[..n], Style::Mono) > room {
        n -= 1;
    }
    draw(tx, y, &entry.text[..n], Style::Mono, text);
}
