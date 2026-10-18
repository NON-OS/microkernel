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

//! What a click on a network row or a search result does: a found network
//! starts its join, a saved one is only highlighted, and a result opens.

use super::search_results::open_selected;
use crate::settings::state::wifi::join::connect_selected;
use crate::settings::state::wifi::pass::clear_passphrase;
use crate::settings::state::State;
use crate::settings::ui::bytes::as_str;
use crate::settings::ui::results;
use crate::settings::ui::results_geom::index_at;
use nonos_app_skeleton::EventOutcome;

/// A click on a found network starts its join, as C does: a secured one
/// opens the passphrase editor, so what is typed next is the passphrase and
/// not the page's letter keys. A click on a saved row only highlights it.
pub(super) fn click_network(state: &mut State, i: usize) {
    let same = state.wifi_cursor == i;
    state.wifi_cursor = i;
    if i >= state.wifi_network_count || (same && state.wifi_pass.active) {
        return;
    }
    clear_passphrase(state);
    connect_selected(state);
}

pub(super) fn results_click(state: &mut State, y: i32) -> EventOutcome {
    let n = results::count(as_str(state.search.text.as_slice()));
    let Some(i) = index_at(y, state.search.scroll, n) else { return EventOutcome::Idle };
    state.search.cursor = i;
    open_selected(state)
}
