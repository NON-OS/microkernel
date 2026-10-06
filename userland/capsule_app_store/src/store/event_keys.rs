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

//! Which key does what.

use nonos_app_skeleton::{
    EventOutcome, KEY_DOWN, KEY_END, KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT,
    KEY_UP,
};

use super::event_actions::act;
use super::event_search::typing;
use super::event_tab::step_tab;

use super::state::State;

const KEY_SLASH: u32 = b'/' as u32;

pub fn on_key(state: &mut State, code: u32) -> EventOutcome {
    // The field takes the keyboard while open.
    if state.search.active {
        if let Some(outcome) = typing(state, code) {
            return outcome;
        }
    }
    let changed = match code {
        KEY_UP => state.move_by(-1),
        KEY_DOWN => state.move_by(1),
        KEY_PAGE_UP => state.move_by(-(state.rows as isize)),
        KEY_PAGE_DOWN => state.move_by(state.rows as isize),
        KEY_HOME => state.move_by(isize::MIN / 2),
        KEY_END => state.move_by(isize::MAX / 2),
        KEY_LEFT => step_tab(state, -1),
        KEY_RIGHT => step_tab(state, 1),
        KEY_SLASH => {
            state.search.open();
            true
        }
        c => return act(state, c),
    };
    match changed {
        true => EventOutcome::Repaint,
        false => EventOutcome::Idle,
    }
}
