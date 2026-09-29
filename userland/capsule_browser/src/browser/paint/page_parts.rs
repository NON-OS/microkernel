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

use crate::browser::omnibox::geometry::{BUBBLE_BAND, CONTENT_TOP};
use crate::browser::omnibox::Damage;
use crate::browser::state::State;

use super::{box_page, bubble, document, rows, scroll_paint};

/* The whole page area, then the bubble over it. */
pub(super) fn page_full(state: &mut State, fb: &mut PaintBuffer) {
    match state.box_doc.as_ref() {
        Some(doc) => box_page::paint(state, doc, fb),
        None => document::paint(state, fb),
    }
    bubble::paint(state, fb);
    state.track.painted_scroll = state.scroll;
}

/* The page parts a partial repaint covers. A page change repaints the
 * page; a scroll shifts the pixels still in view and draws the rest; a
 * bubble change redraws only the band the bubble sits in. Whatever the
 * cheap path cannot do exactly falls back to the whole page. */
pub(super) fn page_parts(state: &mut State, fb: &mut PaintBuffer, parts: Damage) {
    if parts.has(Damage::PAGE) {
        return page_full(state, fb);
    }
    /* A scroll that ended where it started redraws nothing, so a bubble
     * change in the same frame still takes the band repaint. */
    let moved = parts.has(Damage::SCROLL) && state.track.painted_scroll != state.scroll;
    if moved && !scroll_paint::shift(state, fb) {
        return page_full(state, fb);
    }
    if parts.has(Damage::BUBBLE) && !moved {
        let view_h = fb.height.saturating_sub(CONTENT_TOP);
        if !rows::paint_rows(state, fb, view_h.saturating_sub(BUBBLE_BAND), view_h) {
            return page_full(state, fb);
        }
        bubble::paint(state, fb);
    }
}
