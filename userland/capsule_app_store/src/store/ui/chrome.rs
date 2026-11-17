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

//! The head band, its search field and the source tabs.

use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::theme::roles;
use nonos_toolkit::widgets::{text, Field, Rect, State as Look};

use crate::store::search::Search;
use crate::store::state::{State, TABS};

use super::bytes::as_str;
use super::counter::listed;
use super::hover_look::pointed;
use super::metrics::metrics;
use super::tab_strip;
use super::target::Target;

pub fn head(fb: &mut PaintBuffer, state: &State) {
    let (r, m) = (roles(), metrics());
    let top = text::centre_top(m.pad_top, m.head_h, m.title_px);
    text::left(fb, m.pad_x, top, "Marketplace", r.text, m.title_px);
    let right = state.fb_w.saturating_sub(m.pad_x);
    let field = search(fb, &state.search, right);
    let meta_top = text::centre_top(m.pad_top, m.head_h, m.body_px);
    let count = state.visible().len();
    let at = right.saturating_sub(field + if field == 0 { 0 } else { m.pad_x });
    text::right(fb, at, meta_top, as_str(&listed(count)), r.muted, m.body_px);
}

/// Paints the field while it is open or holds a word, and reports how much
/// width it took, so the head can put its count to the left of it rather
/// than underneath.
fn search(fb: &mut PaintBuffer, search: &Search, right: u32) -> u32 {
    if !search.active && search.text().is_empty() {
        return 0;
    }
    let query = as_str(search.text());
    let m = metrics();
    let y = m.pad_top + (m.head_h - m.search_h) / 2;
    let at = Rect { x: right.saturating_sub(m.search_w), y, w: m.search_w, h: m.search_h };
    Field::new(query)
        .placeholder("type to search")
        .caret(search.active.then_some(query.len()))
        .state(Look { focused: search.active, ..Look::default() })
        .draw(fb, at);
    m.search_w
}

/// The source tabs, one strip across the set, with the tab under the pointer
/// washed.
pub fn tabs(fb: &mut PaintBuffer, state: &State) {
    let hovered = (0..TABS.len()).find(|&i| pointed(state, |t| t == Target::Tab(i)).hovered);
    tab_strip::draw(fb, state.fb_w, Some(state.tab), hovered);
}
