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

/* chown and its forms: every file is root's, and stays so. */

use nonos_libc::mk_debug;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::at::resolve_at;
use super::super::meta::stat;

const KEEP: u32 = u32::MAX;

pub(super) const AT_SYMLINK_NOFOLLOW: u64 = 0x100;

/* `fchownat`; `chown`, `lchown` and `fchown` are this at other bases. */
pub fn fchownat(guest: &Guest, dirfd: u64, path: u64, uid: u64, gid: u64) -> u64 {
    let Some(at) = resolve_at(guest, dirfd, path) else {
        return errno::fail(errno::EFAULT);
    };
    if stat::look(&super::super::walk::follow(guest, at, true)).is_none() {
        return errno::fail(errno::ENOENT);
    }
    fchown_ids(uid, gid)
}

/* `fchown` on an open descriptor: only the owner every file already has. */
pub fn fchown_ids(uid: u64, gid: u64) -> u64 {
    match [uid as u32, gid as u32].iter().all(|id| *id == 0 || *id == KEEP) {
        true => errno::ok(0),
        false => refused(b"[LINUX] refused chown: owners are not recorded\n"),
    }
}

pub(crate) fn refused(line: &[u8]) -> u64 {
    let _ = mk_debug(line.as_ptr(), line.len());
    errno::fail(errno::EPERM)
}
