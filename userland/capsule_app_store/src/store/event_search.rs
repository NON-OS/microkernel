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

//! The keyboard while the search field is open.

use nonos_app_skeleton::{EventOutcome, KEY_BACKSPACE, KEY_ENTER, KEY_ESC};

use super::state::State;

pub fn typing(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        // Escape leaves the field rather than closing the window.
        KEY_ESC => {
            state.search.close();
            reset(state);
            Some(EventOutcome::Repaint)
        }
        /*
         * Enter keeps the filter and gives the keyboard back, so the
         * next Enter installs what was found.
         */
        KEY_ENTER => {
            state.search.active = false;
            Some(EventOutcome::Repaint)
        }
        KEY_BACKSPACE => {
            let changed = state.search.pop();
            reset(state);
            Some(match changed {
                true => EventOutcome::Repaint,
                false => EventOutcome::Idle,
            })
        }
        c if (0x20..0x7F).contains(&c) => {
            let changed = state.search.push(c as u8);
            reset(state);
            Some(match changed {
                true => EventOutcome::Repaint,
                false => EventOutcome::Idle,
            })
        }
        _ => None,
    }
}

/// The list under the cursor just changed, so the cursor cannot stay where it
/// was: it would point past the end, or at a row the query no longer keeps.
fn reset(state: &mut State) {
    state.cursor = 0;
    state.scroll = 0;
    state.select();
}
