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
use nonos_app_skeleton::clients::vfs::{read_file, write_file};

use crate::term::cwd::resolve;
use crate::term::state::State;

/// The largest file `>` writes or `>>` adds to. An existing file past it is
/// refused rather than cut short by the append.
pub(crate) const REDIRECT_MAX: usize = 1024 * 1024;
/// What `read_file` says when there is no file to open.
const OPEN_FAILED: &str = "vfs open failed";

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

/// `data` into the file at `path` (already resolved), replacing it or, for
/// `>>`, after what it holds. A file that is not there yet is made.
pub(crate) fn write_out(
    owner: u32,
    path: &[u8],
    data: &[u8],
    append: bool,
) -> Result<(), &'static str> {
    if data.len() > REDIRECT_MAX {
        return Err("more output than a redirect keeps (1 MiB)");
    }
    let mut whole = Vec::new();
    if append {
        /* One byte past the most, so a file too big to add to is seen whole
         * rather than read short and written back cut. */
        match read_file(owner, path, REDIRECT_MAX as u32 + 1) {
            Ok(held) if held.len() + data.len() > REDIRECT_MAX => {
                return Err("would grow past what a redirect keeps (1 MiB)")
            }
            Ok(held) => whole = held,
            /* Nothing there to open: `>>` makes it, as `>` would. A file
             * that opened and then failed to read is never overwritten. */
            Err(OPEN_FAILED) => {}
            Err(e) => return Err(e),
        }
    }
    whole.extend_from_slice(data);
    write_file(owner, path, &whole)
}

pub(crate) fn wrote_to(path: &[u8]) -> Vec<u8> {
    [b"wrote to ".as_slice(), path].concat()
}

pub(crate) fn not_written(path: &[u8], why: &str) -> Vec<u8> {
    [path, b": not written: ".as_slice(), why.as_bytes()].concat()
}
