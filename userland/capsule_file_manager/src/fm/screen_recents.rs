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

use alloc::vec::Vec;

use nonos_app_skeleton::PaintBuffer;
use nonos_libc::mk_time_millis;

use super::file_color::color;
use super::file_kind::kind_of_name;
use super::layout::{CONTENT_X, FOOTER_H, HEADER_H, PAD_X, SECTION_GAP};
use super::recents_group::{group, parent_of, rel_time};
use super::screen_list::Line;
use super::screen_row::{screen_row, section_label, LABEL_ADV, LIST_ROW_H};
use super::sidebar_rows::base_label;
use super::state::State;

/// `recents_group::group` is the single geometry source: it decides which
/// sections exist and what each holds, and this walks the result into the
/// screen's one line list, so an empty bucket is never drawn as a bare heading
/// and a click can never land past the last line drawn.
pub fn recents_lines(state: &State, now: u64) -> Vec<Line> {
    let bottom = state.win_h.saturating_sub(FOOTER_H);
    let mut out = Vec::new();
    let mut y = HEADER_H + 16;
    for (label, rows) in group(now, &state.recents) {
        if y + LIST_ROW_H > bottom {
            break;
        }
        out.push(Line::head(y, LABEL_ADV, label));
        y += LABEL_ADV;
        for (ms, path) in rows {
            if y + LIST_ROW_H > bottom {
                break;
            }
            out.push(Line::row(y, LIST_ROW_H, path, rel_time(now, ms), path.ends_with('/')));
            y += LIST_ROW_H;
        }
        y += SECTION_GAP;
    }
    out
}

pub fn paint_recents(state: &State, fb: &mut PaintBuffer) {
    let x = CONTENT_X + PAD_X;
    let w = fb.width.saturating_sub(CONTENT_X + PAD_X * 2);
    let lines = recents_lines(state, mk_time_millis().max(0) as u64);
    if lines.is_empty() {
        super::screen_row::empty_state(
            fb,
            x,
            HEADER_H + 40,
            w,
            "No recent files",
            "Opening a file records it in the store journal.",
        );
        return;
    }
    for line in &lines {
        let Some(path) = paint_head(fb, x, line) else { continue };
        let title = base_label(path);
        let row = (title.as_str(), parent_of(path), line.meta.as_str());
        screen_row(fb, x, line.y, w, row, line.dir, color(kind_of_name(path)));
    }
}

// Draws a heading line and reports nothing; a row line reports its path so the
// caller draws it instead.
fn paint_head<'a>(fb: &mut PaintBuffer, x: u32, line: &'a Line) -> Option<&'a str> {
    match line.head {
        Some(text) => {
            section_label(fb, x, line.y, text);
            None
        }
        None => Some(line.path.as_str()),
    }
}
