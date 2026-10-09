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

//! Create a directory at an absolute path.

use alloc::vec;

use super::call::call_status;
use super::constants::OP_MKDIR;
use super::owner_body::owner_body;
use super::path;

const EINVAL: i32 = -22;

/// Ok, or why not: the server's errno, or `EINVAL` for a path it will not send.
pub fn mkdir(path: &[u8]) -> Result<(), i32> {
    if !path::is_valid(path) {
        return Err(EINVAL);
    }
    let mut rx = vec![0u8; 64];
    call_status(OP_MKDIR, &owner_body(path), &mut rx).map(|_| ())
}
