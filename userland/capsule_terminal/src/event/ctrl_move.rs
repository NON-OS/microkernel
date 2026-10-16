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

//! The Ctrl keys that move the cursor or zoom the font.

use nonos_app_skeleton::{EventOutcome, KEY_LEFT, KEY_RIGHT};

use super::ctrl_keys::*;
use crate::event::accept_suggestion::accept_suggestion;
use crate::event::bool_to_outcome::bool_to_outcome;
use crate::term::state::State;

pub(super) fn ctrl_move(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        // Ctrl with the arrows moves by word, the motion that sits between
        // stepping one byte and jumping to the end of the line. These arrive
        // here rather than with the other arrows because any Ctrl key is
        // routed to this function first.
        KEY_LEFT => Some(bool_to_outcome(state.line.move_word_left())),
        KEY_RIGHT => Some(bool_to_outcome(state.line.move_word_right())),
        CTRL_A | CTRL_A_LO => {
            state.line.move_home();
            Some(EventOutcome::Repaint)
        }
        CTRL_E | CTRL_E_LO => {
            if !accept_suggestion(state) {
                state.line.move_end();
            }
            Some(EventOutcome::Repaint)
        }
        // Ctrl+= / Ctrl++ zoom the body font in, Ctrl+- / Ctrl+_ out (1..=4).
        ZOOM_IN_EQ | ZOOM_IN_PLUS => {
            state.zoom_req += 1;
            Some(EventOutcome::Repaint)
        }
        ZOOM_OUT_MINUS | ZOOM_OUT_USCORE => {
            state.zoom_req -= 1;
            Some(EventOutcome::Repaint)
        }
        _ => None,
    }
}
