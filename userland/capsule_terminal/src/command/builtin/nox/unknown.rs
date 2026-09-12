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

use alloc::vec::Vec;

use crate::term::state::State;

pub fn run(state: &mut State, verb: &[u8]) {
    let mut line = Vec::new();
    line.extend_from_slice(verb);
    line.extend_from_slice(b": not found");

    // A mistyped command nearly always has an obvious neighbour, and naming it
    // ends the problem here rather than sending the reader to look it up. The
    // candidates are every name that would actually have run.
    let candidates = crate::event::complete::all_names();
    match crate::command::suggest::nearest_two(verb, candidates.into_iter()) {
        (Some(near), Some(alt)) => {
            line.extend_from_slice(b". did you mean '");
            line.extend_from_slice(near);
            line.extend_from_slice(b"' or '");
            line.extend_from_slice(alt);
            line.extend_from_slice(b"'?");
        }
        (Some(near), None) => {
            line.extend_from_slice(b". did you mean '");
            line.extend_from_slice(near);
            line.extend_from_slice(b"'?");
        }
        _ => line.extend_from_slice(b". 'help' lists what there is"),
    }
    state.scrollback.push_error(&line);
}
