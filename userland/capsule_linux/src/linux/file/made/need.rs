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
 * Which calls need the family's view lent before they are answered: the
 * ones that may name a path under /proc, read a /proc descriptor, or
 * release a lock another process may share. Every other call skips it.
 */

use crate::linux::guest::{Guest, Kind};

/*
 * open, stat, lstat, access, execve, chdir, readlink, chmod, statfs,
 * utime, getppid, the *at forms, and close and the calls that close.
 */
const ALWAYS: [u64; 23] = [
    2, 3, 4, 6, 21, 33, 59, 80, 89, 90, 110, 132, 137, 235, 257, 262, 267, 268, 269, 280, 292, 332,
    439,
];

pub fn needs_view(guest: &Guest, nr: u64, a: [u64; 6]) -> bool {
    if ALWAYS.contains(&nr) {
        return true;
    }
    let proc_fd = |fd: u64| {
        guest
            .fds
            .get(fd as usize)
            .is_some_and(|f| f.kind == Kind::File && f.path.starts_with(b"/proc"))
    };
    /* read, fstat, pread64 and readv */
    matches!(nr, 0 | 5 | 17 | 19) && proc_fd(a[0])
}
