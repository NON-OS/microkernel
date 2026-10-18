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

//! The Ctrl keys that cut, delete and put back text on the line.

use nonos_app_skeleton::EventOutcome;

use super::ctrl_keys::*;
use super::ctrl_move::ctrl_move;
use crate::event::bool_to_outcome::bool_to_outcome;
use crate::term::state::State;

pub(super) fn ctrl_edit(state: &mut State, code: u32) -> Option<EventOutcome> {
    match code {
        CTRL_U | CTRL_U_LO => {
            state.line.kill_line();
            Some(EventOutcome::Repaint)
        }
        CTRL_W | CTRL_W_LO => {
            state.line.delete_word();
            Some(EventOutcome::Repaint)
        }
        CTRL_K | CTRL_K_LO => {
            state.line.kill_to_end();
            Some(EventOutcome::Repaint)
        }
        // Ctrl-D deletes the character under the cursor, the forward twin of
        // backspace. It does not close the terminal on an empty line: that is
        // what `exit` is for, and a window that vanishes on a mistyped key is
        // a window nobody trusts to hold work.
        CTRL_D | CTRL_D_LO => Some(bool_to_outcome(state.line.delete())),
        // Ctrl-Y puts back what the last kill key cut. Without it Ctrl-U,
        // Ctrl-W and Ctrl-K are destructive with no way back, which is the
        // one thing a line editor must never be.
        CTRL_Y | CTRL_Y_LO => {
            state.line.yank();
            Some(EventOutcome::Repaint)
        }
        _ => ctrl_move(state, code),
    }
}
