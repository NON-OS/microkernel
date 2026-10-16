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

//! A pulled file written to the store, said and counted either way.

use super::super::{progress, store};
use crate::term::state::State;

/// `body` into the store at `dest`.
pub(super) fn store_body(
    state: &mut State,
    pid: u32,
    dest: &[u8],
    body: &[u8],
    tally: &mut progress::Tally,
) -> bool {
    match store::write(pid, dest, body) {
        Ok(()) => {
            state.scrollback.push_line(&progress::file_line(dest, body.len()));
            tally.ok += 1;
            true
        }
        Err(e) => {
            state.scrollback.push_error(e.as_bytes());
            tally.failed += 1;
            false
        }
    }
}
