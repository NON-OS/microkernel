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

/*
 * The errno for a store request that failed, from the reason the store
 * gave. A full store is ENOSPC and a file too large for it EFBIG, as a
 * Linux filesystem says; a reason with no Linux name is EIO.
 */

use crate::linux::abi::errno;

pub fn errno_of(reason: &str) -> i64 {
    match reason {
        "no space left" => errno::ENOSPC,
        "too large" => errno::EFBIG,
        "read-only file system" => errno::EROFS,
        "not found" => errno::ENOENT,
        "access denied" => errno::EACCES,
        "already exists" => errno::EEXIST,
        "is a directory" => errno::EISDIR,
        "directory not empty" => errno::ENOTEMPTY,
        _ => errno::EIO,
    }
}
