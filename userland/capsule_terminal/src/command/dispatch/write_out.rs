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

//! Bytes into a store file for `>` or `>>`, within what a redirect keeps.

use alloc::vec::Vec;
use nonos_app_skeleton::clients::vfs::{read_file, write_file};

use super::REDIRECT_MAX;

/// What `read_file` says when there is no file to open.
const OPEN_FAILED: &str = "vfs open failed";

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
