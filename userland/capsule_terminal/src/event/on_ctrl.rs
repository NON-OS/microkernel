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

use nonos_app_skeleton::{EventOutcome, MOD_SHIFT};

use super::copy_line::copy_line;
use super::paste_clipboard::paste_clipboard;
use super::search::{search_cancel, search_step};
use crate::term::state::State;

#[path = "ctrl_edit.rs"]
mod ctrl_edit;
#[path = "ctrl_keys.rs"]
mod ctrl_keys;
#[path = "ctrl_move.rs"]
mod ctrl_move;
use ctrl_edit::ctrl_edit;
use ctrl_keys::*;

pub fn on_ctrl(state: &mut State, code: u32, flags: u16) -> Option<EventOutcome> {
    let shift = flags & MOD_SHIFT != 0;
    match code {
        CTRL_V | CTRL_V_LO => Some(paste_clipboard(state)),
        CTRL_C | CTRL_C_LO if shift => Some(copy_line(state)),
        // The same key starts a search and steps it, which is what every
        // shell binds it to: the key is pressed again because the match on
        // screen is not the one that was meant.
        CTRL_R | CTRL_R_LO => {
            search_step(state);
            Some(EventOutcome::Repaint)
        }
        // Leaving a search puts the line back as it was, rather than killing
        // a line the reader never chose to change.
        CTRL_C | CTRL_C_LO if state.search.is_some() => {
            search_cancel(state);
            Some(EventOutcome::Repaint)
        }
        CTRL_L | CTRL_L_LO => {
            state.scrollback.clear();
            state.scrollback.jump_bottom();
            Some(EventOutcome::Repaint)
        }
        CTRL_C | CTRL_C_LO => Some(super::interrupt::interrupt(state)),
        _ => ctrl_edit(state, code),
    }
}
