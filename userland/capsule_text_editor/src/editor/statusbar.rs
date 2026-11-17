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

//! The bottom status bar, one `Row`: the active document's message on the
//! left and its caret position and language, or its page view and zoom, on
//! the right. While the find bar or a path prompt is open the left shows what
//! is being typed, a bar marking the field being edited.

use alloc::format;
use alloc::string::String;

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{draw, Ink, Rect, Row, Separator};

use super::canvas::page_index;
use super::language::language_name;
use super::layout::{footer_h, zoom_px};
use super::mode::Mode;
use super::paint::caret_position;
use super::state::State;
use crate::doc::counts::word_count;

const GAP: &str = "    ";

pub(super) fn paint_status(fb: &mut PaintBuffer, doc: &State, width: u32, height: u32) {
    let h = footer_h();
    let band = Rect { x: 0, y: height.saturating_sub(h), w: width, h };
    draw::fill(fb, band, roles().sunken);
    Separator::new().draw(fb, Rect { h: scale().hair(), ..band });
    let (left, right) = status_text(doc);
    Row::new(&left).value(&right).value_ink(Ink::Muted).draw(fb, band);
}

fn status_text(doc: &State) -> (String, String) {
    let status = core::str::from_utf8(doc.status).unwrap_or("");
    if doc.find_active {
        let bar = |on: bool| if on { "|" } else { "" };
        let mut left = format!("Find: {}{}", doc.find_buf, bar(!doc.replace_active));
        if doc.replace_active || !doc.replace_buf.is_empty() {
            left += &format!("  Replace: {}{}", doc.replace_buf, bar(doc.replace_active));
        }
        return (left, format!("{} matches", doc.find_count()));
    }
    if doc.prompt.is_some() {
        let typed = core::str::from_utf8(&doc.prompt_path[..doc.prompt_len]).unwrap_or("");
        return (format!("{}  {}|", status, typed), String::new());
    }
    if doc.mode == Mode::Document {
        let page = format!("Page {} of {}", page_index(doc) + 1, doc.pages.len().max(1));
        let words = format!("{} words", grouped(word_count(&doc.doc)));
        let lead = if status.is_empty() { String::new() } else { format!("{}{}", status, GAP) };
        let zoom = format!("Page view{}{}%", GAP, zoom_percent(doc.font_scale));
        return (format!("{}{}{}{}", lead, page, GAP, words), zoom);
    }
    let (line, col) = caret_position(doc);
    let path = core::str::from_utf8(&doc.path[..doc.path_len]).unwrap_or("");
    let info =
        format!("Ln {}, Col {}{}{}{}UTF-8", line + 1, col + 1, GAP, language_name(path), GAP);
    (String::from(status), info)
}

// Zoom relative to the default scale 2, whose 15px body reads as 100%.
fn zoom_percent(scale: u32) -> u32 {
    zoom_px(scale) * 100 / zoom_px(2)
}

// Thousands separators, so a long document reads "1,245 words".
fn grouped(n: usize) -> String {
    let digits = format!("{}", n);
    let mut out = String::new();
    for (i, c) in digits.char_indices() {
        if i > 0 && (digits.len() - i) % 3 == 0 {
            out.push(',');
        }
        out.push(c);
    }
    out
}
