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

/*
 * Moving along the line and through history: the arrows, Home and End on
 * the line, Page Up and Page Down on the scrollback.
 */

use nonos_app_skeleton::{
    EventOutcome, KEY_END, KEY_HOME, KEY_LEFT, KEY_PAGE_DOWN, KEY_PAGE_UP, KEY_RIGHT,
};

use super::bool_to_outcome::bool_to_outcome;
use crate::term::state::State;

pub fn on_nav(state: &mut State, code: u32) -> Option<EventOutcome> {
    Some(match code {
        KEY_LEFT => bool_to_outcome(state.line.move_left()),
        KEY_RIGHT => bool_to_outcome(state.line.move_right()),
        KEY_HOME => {
            state.line.move_home();
            EventOutcome::Repaint
        }
        KEY_END => {
            state.line.move_end();
            EventOutcome::Repaint
        }
        KEY_PAGE_UP => {
            state.scrollback.scroll_up(super::key_first::page(state));
            EventOutcome::Repaint
        }
        KEY_PAGE_DOWN => {
            state.scrollback.scroll_down(super::key_first::page(state));
            EventOutcome::Repaint
        }
        _ => return None,
    })
}
