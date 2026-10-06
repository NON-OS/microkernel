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

use nonos_app_skeleton::EventOutcome;

use crate::settings::state::refresh_wifi::run_wifi_scan;
use crate::settings::state::wifi_join::{clear_passphrase, connect_selected};
use crate::settings::state::{search_clear, searching, set_section, track_scroll, view_h, State};
use crate::settings::ui::bytes::as_str;
use crate::settings::ui::hit::{at, Hit};
use crate::settings::ui::metrics::SIDEBAR_W;
use crate::settings::ui::nav_geom;
use crate::settings::ui::results;
use crate::settings::ui::results_geom::index_at;

use super::on_search_key::open_selected;
use super::pointer_row::activate;

pub(super) fn on_pointer(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < 0 || y < 0 {
        return EventOutcome::Idle;
    }
    let was_focused = core::mem::replace(&mut state.search_focused, false);
    let outcome = route(state, x, y);
    if was_focused && outcome == EventOutcome::Idle {
        return EventOutcome::Repaint;
    }
    outcome
}

fn route(state: &mut State, x: i32, y: i32) -> EventOutcome {
    if x < SIDEBAR_W as i32 {
        let Some(section) = nav_geom::at(x, y) else { return EventOutcome::Idle };
        search_clear(state);
        set_section(state, section);
        return EventOutcome::Repaint;
    }
    if y >= view_h(state) as i32 {
        return EventOutcome::Idle;
    }
    if searching(state) {
        return results_click(state, y);
    }
    let pane_w = state.win_w.saturating_sub(SIDEBAR_W);
    let scroll = state.scroll_px[state.section.index()];
    match at(state, x - SIDEBAR_W as i32, y, scroll, pane_w) {
        Hit::Field { index, control } => {
            state.cursor[state.section.index()] = index;
            track_scroll(state);
            activate(state, control)
        }
        Hit::Network(i) => {
            click_network(state, i);
            EventOutcome::Repaint
        }
        Hit::Scan => {
            run_wifi_scan(state);
            EventOutcome::Repaint
        }
        Hit::None => EventOutcome::Idle,
    }
}

/// A click on a found network starts its join, as C does: a secured one
/// opens the passphrase editor, so what is typed next is the passphrase and
/// not the page's letter keys. A click on a saved row only highlights it.
fn click_network(state: &mut State, i: usize) {
    let same = state.wifi_cursor == i;
    state.wifi_cursor = i;
    if i >= state.wifi_network_count || (same && state.wifi_pass_active) {
        return;
    }
    clear_passphrase(state);
    connect_selected(state);
}

fn results_click(state: &mut State, y: i32) -> EventOutcome {
    let n = results::count(as_str(state.search.as_slice()));
    let Some(i) = index_at(y, state.search_scroll, n) else { return EventOutcome::Idle };
    state.search_cursor = i;
    open_selected(state)
}
