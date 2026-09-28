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
 * open, stat, lstat, access, execve, truncate, chdir, readlink, chmod,
 * statfs, utime, getppid, the xattr calls, the *at forms and openat2, and
 * close and the calls that close.
 */
const ALWAYS: [u64; 38] = [
    2, 3, 4, 6, 21, 33, 59, 76, 80, 89, 90, 110, 132, 137, 188, 189, 190, 191, 192, 193, 194, 195,
    196, 197, 198, 199, 235, 257, 262, 267, 268, 269, 280, 292, 332, 436, 437, 439,
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
    match nr {
        /* read, fstat, pread64, readv, preadv, preadv2, copy_file_range */
        0 | 5 | 17 | 19 | 295 | 326 | 327 => proc_fd(a[0]),
        /* sendfile reads its second descriptor */
        40 => proc_fd(a[1]),
        _ => false,
    }
}
