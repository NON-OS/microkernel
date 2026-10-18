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

//! Turning the typed line into the one that runs, `!` forms resolved.

use crate::command;
use crate::term::dimensions::LINE_MAX;
use crate::term::state::State;

use super::super::echo_line::echo_line;

/// Copy the line into `entered`, history references expanded, and say how
/// long it is; `None` when it names an entry that is not there, which has
/// been reported and runs nothing.
pub(super) fn expand_line(state: &mut State, entered: &mut [u8; LINE_MAX]) -> Option<usize> {
    // A `!` form is resolved before anything else sees the line, so what is
    // echoed, recorded in history and run are all the same text. Expanding
    // later would put one command on screen and another through the parser.
    let typed = state.line.as_bytes();
    let mut n = typed.len().min(LINE_MAX);
    entered[..n].copy_from_slice(&typed[..n]);
    /*
     * A line that starts with `qwen` is a question, and a `!word` in it is
     * part of what is asked, not a history reference.
     */
    let expanded = if command::builtin::qwen::is_line(state.line.as_bytes()) {
        None
    } else {
        crate::term::history::expand(state.line.as_bytes(), &state.history)
    };
    match expanded {
        // The expansion is what gets echoed, which is the whole safety of the
        // feature: the reader sees the command that is about to run, not the
        // shorthand they typed for it.
        Some(Ok(line)) => {
            n = line.len().min(LINE_MAX);
            entered[..n].copy_from_slice(&line[..n]);
        }
        Some(Err(_)) => {
            // Naming an entry that is not there runs nothing. Silently
            // dropping the `!` would run the rest of the line, which is how
            // history expansion earns its reputation.
            echo_line(state, &entered[..n]);
            state.scrollback.push_line(b"no matching history entry");
            state.line.clear();
            state.history.reset_cursor();
            state.last_status = 1;
            state.scrollback.jump_bottom();
            return None;
        }
        None => {}
    }
    Some(n)
}
