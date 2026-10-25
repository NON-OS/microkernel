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

//! Searching back through what has been run.

use crate::term::search_step::{begin, cancel, older, runs};
use crate::term::state::State;

/// Begin a search, or step to the next older match if one is running.
///
/// The same key does both, which is what every shell binds it to: the reader
/// presses it again because the match shown is not the one they meant.
pub fn search_step(state: &mut State) {
    let search = match state.search.take() {
        None => begin(&mut state.line, &state.history),
        Some(mut search) => {
            older(&mut search, &state.history, &mut state.line);
            search
        }
    };
    state.search = Some(search);
}

/// Enter under a search: leave it keeping the match, and say whether the line
/// may run. A needle with no match keeps the search open and runs nothing.
pub fn search_enter(state: &mut State) -> bool {
    match &state.search {
        None => true,
        Some(search) if runs(search) => {
            state.search = None;
            true
        }
        Some(_) => false,
    }
}

/// Leave the search, restoring the line as it was.
pub fn search_cancel(state: &mut State) {
    if let Some(search) = state.search.take() {
        cancel(search, &mut state.line);
    }
}
