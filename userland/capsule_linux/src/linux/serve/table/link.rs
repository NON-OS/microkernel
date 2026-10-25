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
 * Links, renames, owners, times and nodes. The plain calls are their *at
 * forms at AT_FDCWD, so each property is decided in one place.
 */

use crate::linux::abi::nr_path as np;
use crate::linux::file;
use crate::linux::file::flags::AT_FDCWD as CWD;
use crate::linux::guest::Guest;

pub fn link_ops(guest: &mut Guest, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        np::SYMLINK => file::symlinkat(guest, a[0], CWD, a[1]),
        np::SYMLINKAT => file::symlinkat(guest, a[0], a[1], a[2]),
        np::LINK => file::linkat(guest, CWD, a[0], CWD, a[1]),
        np::LINKAT => file::linkat(guest, a[0], a[1], a[2], a[3]),
        np::READLINKAT => file::readlinkat(guest, a[0], a[1], a[2], a[3]),
        np::RENAMEAT => file::renameat2(guest, a[0], a[1], a[2], a[3], 0),
        np::RENAMEAT2 => file::renameat2(guest, a[0], a[1], a[2], a[3], a[4]),
        np::CHOWN | np::LCHOWN => file::fchownat(guest, CWD, a[0], a[1], a[2]),
        np::FCHOWNAT => file::fchownat(guest, a[0], a[1], a[2], a[3]),
        np::FCHOWN => file::fchown_ids(a[1], a[2]),
        np::UTIMENSAT => file::utimensat(guest, a[0], a[1], a[2], a[3]),
        np::UTIMES => file::utimes(guest, a[0], a[1], true),
        np::UTIME => file::utimes(guest, a[0], a[1], false),
        np::MKNOD => file::mknodat(guest, CWD, a[0], a[1]),
        np::MKNODAT => file::mknodat(guest, a[0], a[1], a[2]),
        _ => return None,
    })
}
