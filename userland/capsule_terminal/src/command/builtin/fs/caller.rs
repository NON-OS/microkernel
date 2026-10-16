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

//! Who the file commands speak to the vfs as, and the paths they ask about.

use crate::term::state::State;

// The terminal's real pid, resolved once. The vfs server rejects a claimed
// owner pid that differs from the sender, so this must be our own.
pub(in super::super) fn pid(state: &mut State) -> u32 {
    if state.owner_pid == 0 {
        state.owner_pid = nonos_libc::mk_getpid();
    }
    state.owner_pid
}

// Absolute path for an argument, resolved against the working directory.
pub(in super::super) fn abspath(state: &State, arg: &[u8]) -> alloc::vec::Vec<u8> {
    crate::term::cwd::resolve(state.cwd.as_bytes(), arg)
}
