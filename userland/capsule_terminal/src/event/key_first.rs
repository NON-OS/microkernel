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

//! What a key reaches before the shell's line editor: copy, paste and
//! search, an open search bar, and a running program.

use nonos_app_skeleton::{EventOutcome, InputEvent};

use crate::term::state::State;

pub fn key_first(state: &mut State, event: InputEvent) -> Option<EventOutcome> {
    if let Some(out) = super::clip::clip_key(state, &event) {
        return Some(out);
    }
    if state.find.is_some() {
        return Some(super::find_bar::key(state, event));
    }
    // Typing moves on from whatever was picked.
    state.sel = None;
    /*
     * A running program gets the keys first, Ctrl ones included: a full
     * screen program binds them itself.
     */
    super::fg_keys::fg_key(state, event)
}

/// A page of history: the screen less two lines of overlap.
pub fn page(state: &State) -> usize {
    state.scrollback.vt.rows().saturating_sub(2).max(1)
}
