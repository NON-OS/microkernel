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

use nonos_app_skeleton::input::{KEY_DOWN, KEY_UP};
use nonos_app_skeleton::EventOutcome;

use crate::calc::mode::Mode;
use crate::calc::state::State;
use crate::calc::ui::history_geom;

pub fn click(state: &mut State, x: i32, y: i32) -> bool {
    let (w, h) = state.view;
    let row = match history_geom::at(w, h, x, y) {
        Some(found) => found,
        None => return false,
    };
    let index = history_geom::entry_at(state.history_scroll, row);
    let value = match state.history.get(index) {
        Some(entry) => entry.value,
        None => return false,
    };
    state.set_mode(Mode::Basic);
    state.display = value;
    true
}

/// The wheel and Up and Down scroll the page, so every calculation the ring
/// holds can be reached, not only those that fit.
pub fn scroll(state: &mut State, rows: i32) -> EventOutcome {
    let next =
        history_geom::scroll_by(state.history_scroll, rows, state.history.len(), state.view.1);
    if next == state.history_scroll {
        return EventOutcome::Idle;
    }
    state.history_scroll = next;
    state.hover = None;
    EventOutcome::Repaint
}

/// Up and Down on the History page, or None for any other key.
pub fn key(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        KEY_UP => Some(scroll(state, -1)),
        KEY_DOWN => Some(scroll(state, 1)),
        _ => None,
    }
}
