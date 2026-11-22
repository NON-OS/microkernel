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

use crate::browser::layout::boxmodel::to_css;
use crate::browser::omnibox::rescale;
use crate::browser::state::State;

/* Hold the window's scroll to the page just laid out at `q` quarters: a
 * page that shrank leaves no blank space under it, and a scale change
 * keeps the reader at the same place. The document's scroll_y follows
 * only while it still says where the window was, so a scrollTo the page
 * asked for and the window has not yet followed is kept for
 * scroll_by::follow_script_scroll to hold to this layout. */
pub(super) fn settle(state: &mut State, q: u32) {
    let content_h = state.box_doc.as_ref().map_or(0, |d| d.content_h);
    let (was, was_q) = (state.scroll, state.laid_q);
    state.scroll = rescale(was, was_q, q, content_h, state.viewport_h);
    state.laid_q = q;
    if state.scroll == was {
        return;
    }
    let now = page_y(state.scroll, q);
    if let Some(dom) = state.page_dom.as_mut().filter(|d| d.scroll_y == page_y(was, was_q)) {
        dom.scroll_y = now;
    }
    crate::browser::image::requeue_visible(state);
}

/* A device px offset at `q` quarters in the CSS px a script reads. */
fn page_y(scroll: u32, q: u32) -> u32 {
    to_css(scroll.min(i32::MAX as u32) as i32, q).max(0) as u32
}
