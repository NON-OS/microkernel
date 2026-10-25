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

//! Where the finished lines go: the screen, a file, or nowhere.

use alloc::vec::Vec;

use super::super::redirect::Sink;
use super::super::write_redirect::write_redirect;
use crate::term::state::State;

/// The lines the command left, where the line's redirect sends them.
pub(super) fn deliver(state: &mut State, output: Sink<'_>, lines: Vec<Vec<u8>>) {
    match output {
        Sink::File { path, append } => write_redirect(state, &lines, append, path),
        Sink::Null => {}
        Sink::Screen => {
            for line in &lines {
                state.scrollback.push_line(line);
            }
        }
    }
}
