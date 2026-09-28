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
 * Calls that only look at a file: stat and its forms, access, statfs,
 * statx and readlink. Reached from the file table when it has no arm.
 */

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::file;
use crate::linux::file::flags;
use crate::linux::guest::Guest;

/* fstatat's AT_SYMLINK_NOFOLLOW, which lstat is. */
const NOFOLLOW: u64 = 0x100;

pub fn meta_ops(guest: &mut Guest, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        nr::FSTAT => file::fstat(guest, a[0], a[1]),
        nr::STAT => file::newfstatat(guest, flags::AT_FDCWD, a[0], a[1], 0),
        nr::LSTAT => file::newfstatat(guest, flags::AT_FDCWD, a[0], a[1], NOFOLLOW),
        nr::NEWFSTATAT => file::newfstatat(guest, a[0], a[1], a[2], a[3]),
        np::FACCESSAT => file::faccessat(guest, a[0], a[1], a[2], 0),
        np::FACCESSAT2 => file::faccessat(guest, a[0], a[1], a[2], a[3]),
        np::STATFS => file::statfs(guest, a[0], a[1]),
        np::FSTATFS => file::fstatfs(guest, a[0], a[1]),
        np::STATX => file::statx(guest, a[0], a[1], a[2], a[4]),
        nr::ACCESS => file::access(guest, a[0], a[1]),
        nr::READLINK => file::readlinkat(guest, flags::AT_FDCWD, a[0], a[1], a[2]),
        _ => return None,
    })
}
