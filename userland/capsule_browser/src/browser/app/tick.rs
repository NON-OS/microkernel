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

use crate::browser::state::State;

/* One tick of the browser: start a navigation the reader asked for, let the
 * network move, run the page's timers, then note what of all that shows.
 * The runner repaints only when something visible changed since the last
 * paint, so a page at rest costs no pixels at all. */
pub(super) fn tick_body(state: &mut State) -> bool {
    super::nav::start_pending(state);
    super::pumps::fetch_tick(state);
    crate::browser::event::js_tick(state);
    super::print::note_changes(state);
    state.track.paint_gen != state.track.painted_gen
}
