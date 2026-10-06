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

//! Delete a desktop entry: a directory is removed with rmdir, with everything
//! in it (the recursive byte), as the Delete prompt says; a file with unlink.

use alloc::vec;

use super::call::call_status;
use super::constants::{OP_RMDIR, OP_UNLINK};
use super::owner_body::owner_body;
use super::path;

const EINVAL: i32 = -22;

/// vfs_pool's rmdir removes a directory's whole subtree when the byte after
/// the path is non-zero, and refuses a non-empty one otherwise.
const RECURSIVE: u8 = 1;

/// Ok, or why not: the server's errno, or the shell's own for a path it
/// will not send (`EINVAL`) or a call nothing answered.
pub fn remove(path: &[u8], is_dir: bool) -> Result<(), i32> {
    if !path::is_valid(path) {
        return Err(EINVAL);
    }
    let mut rx = vec![0u8; 64];
    if is_dir {
        let mut body = owner_body(path);
        body.push(RECURSIVE);
        return call_status(OP_RMDIR, &body, &mut rx).map(|_| ());
    }
    call_status(OP_UNLINK, &owner_body(path), &mut rx).map(|_| ())
}
