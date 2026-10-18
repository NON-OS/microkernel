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

use crate::term::cwd::resolve;
use crate::term::state::State;

#[path = "write_out.rs"]
mod write_out;

pub(crate) use write_out::write_out;

/// The largest file `>` writes or `>>` adds to. An existing file past it is
/// refused rather than cut short by the append.
pub(crate) const REDIRECT_MAX: usize = 1024 * 1024;

// Write a captured command's output to a VFS file. `>` overwrites; `>>`
// reads the existing file first and appends. The terminal owner pid is
// resolved once and cached on the state, the same way the file commands
// do it.
pub(super) fn write_redirect(state: &mut State, lines: &[Vec<u8>], append: bool, path_arg: &[u8]) {
    let path = resolve(state.cwd.as_bytes(), path_arg);
    let mut data = Vec::new();
    for line in lines {
        data.extend_from_slice(line);
        data.push(b'\n');
    }
    match write_out(state.owner_pid, &path, &data, append) {
        Ok(()) => state.scrollback.push_line(&wrote_to(&path)),
        Err(e) => {
            state.scrollback.push_error(&not_written(&path, e));
            state.last_status = 1;
        }
    }
}

pub(crate) fn wrote_to(path: &[u8]) -> Vec<u8> {
    [b"wrote to ".as_slice(), path].concat()
}

pub(crate) fn not_written(path: &[u8], why: &str) -> Vec<u8> {
    [path, b": not written: ".as_slice(), why.as_bytes()].concat()
}
