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

//! The left pane: its header or the search field, then the chats.

use nonos_app_skeleton::PaintBuffer;

use super::bubble::width;
use super::metrics::{HEADER_H, PAD, ROW_H, SMALL_PX, TITLE_PX};
use super::paint_field::{field, Field};
use super::paint_row::{row, Row};
use super::view::View;
use crate::model::focus::Focus;

const SEARCH_HINT: &str = "ctrl+k search";

fn header(fb: &mut PaintBuffer, v: &View, w: u32) {
    let (m, c, model) = (v.m, &v.c, v.model);
    let pad = m.px(PAD);
    if model.focus == Focus::Search {
        let f = Field { x: pad, y: m.px(10), w: w - 2 * pad, h: m.px(34), focused: true };
        field(fb, &f, &model.search, "search chats", m, c);
        return;
    }
    fb.text_ttf(pad as i32, m.px(15) as i32, "Chats", c.text, TITLE_PX);
    let hint_x = w.saturating_sub(pad + width(SEARCH_HINT, SMALL_PX));
    fb.text_ttf(hint_x as i32, m.px(20) as i32, SEARCH_HINT, c.quiet, SMALL_PX);
}

/// The first row shown, so the selected row stays in view.
fn first_row(selected: Option<usize>, fit: usize) -> usize {
    selected.map_or(0, |s| (s + 1).saturating_sub(fit))
}

pub fn list(fb: &mut PaintBuffer, v: &View, w: u32, h: u32) {
    let (m, c, model) = (v.m, &v.c, v.model);
    fb.fill_rect(0, 0, w, h, c.pane);
    header(fb, v, w);
    let top = m.px(HEADER_H);
    fb.fill_rect(0, top - 1, w, 1, c.rule);
    let row_h = m.px(ROW_H);
    let shown = model.visible();
    let here = model.selected.and_then(|s| model.find(s));
    let pos = here.and_then(|i| shown.iter().position(|&s| s == i));
    let fit = (h.saturating_sub(top) / row_h.max(1)) as usize;
    for (n, &i) in shown.iter().enumerate().skip(first_row(pos, fit)).take(fit) {
        let y = top + (n - first_row(pos, fit)) as u32 * row_h;
        let r = Row { x: 0, y, w, h: row_h, selected: Some(i) == here };
        row(fb, &r, &model.chats[i], m, c);
    }
    if shown.is_empty() {
        let says = if model.chats.is_empty() { "no chats yet" } else { "no chat matches" };
        fb.text_ttf(m.px(PAD) as i32, (top + m.px(PAD)) as i32, says, c.muted, SMALL_PX + 1.0);
    }
}
