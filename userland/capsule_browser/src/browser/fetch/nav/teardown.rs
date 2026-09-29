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

//! Letting go of the page on screen when the next one commits.

use crate::browser::fetch::net_wire::NetWire;
use crate::browser::fetch::pool::Idle;
use crate::browser::net::Source;
use crate::browser::state::State;

/*
 * This used to happen when a navigation began, which blanked the page the
 * moment Enter was pressed and left nothing to look at or click while the
 * next one connected. The old page now stays until the new response is in
 * hand. Kept connections survive: the next page often asks the same hosts.
 */
/// Stop the old page's fetches and forget what it had queued and decoded.
pub(in crate::browser::fetch) fn teardown(state: &mut State, w: &mut NetWire) {
    state.pool.cancel_live(w);
    if let Some(kept) = state.keep.take() {
        let idle = Idle::kept(kept, w.now_ms());
        state.pool.park(w, idle);
    }
    state.css_queue.clear();
    state.script_queue.clear();
    state.image_queue.clear();
    state.images.reset();
    state.font_queue.clear();
    state.font_seen.clear();
    crate::browser::fonts::clear();
    state.page_css.clear();
    state.css_cache = None;
    state.scroll = 0;
    state.focus = None;
}
