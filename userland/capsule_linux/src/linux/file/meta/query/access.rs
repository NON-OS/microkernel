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

/* access and faccessat. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::{at, cache, path, resolve};
use super::super::node::S_IFDIR;

const X_OK: u64 = 1;

const W_OK: u64 = 2;

pub fn access(guest: &Guest, path_ptr: u64, mode: u64) -> u64 {
    faccessat(guest, super::super::super::flags::AT_FDCWD, path_ptr, mode, 0)
}

/*
 * The guest is root, so only two things stand in the way: a read-only
 * mount for W_OK, and a file no one may execute for X_OK.
 */
pub fn faccessat(guest: &Guest, dirfd: u64, path_ptr: u64, mode: u64, flags: u64) -> u64 {
    if mode & !7 != 0 {
        return errno::fail(errno::EINVAL);
    }
    let m = match super::super::stat::meta_at(
        guest,
        dirfd,
        path_ptr,
        flags & super::super::stat::AT_SYMLINK_NOFOLLOW,
    ) {
        Ok(m) => m,
        Err(e) => return errno::fail(e),
    };
    let is_dir = m.mode & 0o170000 == S_IFDIR;
    if mode & X_OK != 0 && !is_dir && m.mode & 0o111 == 0 {
        return errno::fail(errno::EACCES);
    }
    if mode & W_OK != 0 && !writable(guest, dirfd, path_ptr) {
        return errno::fail(errno::EROFS);
    }
    errno::ok(0)
}

fn writable(guest: &Guest, dirfd: u64, path_ptr: u64) -> bool {
    let Some(name) = path::read_path(guest, path_ptr) else { return false };
    let Ok(named) = at::named_at(guest, dirfd, &name) else { return false };
    let full = guest.links.follow(named, true);
    cache::held(&full) || resolve::key(&full).writable().is_ok()
}
