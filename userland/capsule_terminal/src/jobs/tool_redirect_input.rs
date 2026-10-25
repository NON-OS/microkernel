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

//! Reading the file `< f` names, whole, as a program's input.

use alloc::vec::Vec;

use nonos_app_skeleton::clients::vfs::read_file;

use crate::jobs::stdin_queue::StdinQueue;
use crate::term::cwd::resolve;
use crate::term::state::State;

/// The file `< path` names, whole, or the line saying why not.
pub(super) fn read_input(state: &State, name: &[u8], path: &[u8]) -> Result<Vec<u8>, Vec<u8>> {
    let full = resolve(state.cwd.as_bytes(), path);
    let at = |why: &[u8]| [name, b": ", &full[..], b": ", why].concat();
    /* One byte past the most, so a file too big is seen, never cut short. */
    match read_file(state.owner_pid, &full, StdinQueue::FILE_MAX as u32 + 1) {
        Ok(bytes) if bytes.len() > StdinQueue::FILE_MAX => {
            Err(at(b"larger than a program's input takes (1 MiB)"))
        }
        Ok(bytes) => Ok(bytes),
        Err(e) => Err(at(e.as_bytes())),
    }
}
