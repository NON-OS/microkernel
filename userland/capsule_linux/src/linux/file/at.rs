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

/* A `dirfd` and a path, resolved to one absolute name. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::flags::AT_FDCWD;
use super::path::name_of;

/*
 * The name a *at call acts on, or Linux's errno: EFAULT, ENAMETOOLONG or
 * ENOENT for the name itself (`path::name_of`), EBADF or ENOTDIR for the
 * directory descriptor. Each used to be EFAULT.
 */
pub fn resolve_at(guest: &Guest, dirfd: u64, path: u64) -> Result<Vec<u8>, i64> {
    let name = name_of(guest, path)?;
    let full = named_at(guest, dirfd, &name)?;
    /* The *at calls act on the name, so its own last component is not followed. */
    Ok(super::walk::follow(guest, full, false))
}

/*
 * The name `name` gives against `dirfd`, joined and nothing resolved yet:
 * `..` and links are walked in order by `walk::follow`. A directory
 * descriptor keeps the path it was opened at, so a chdir made since does
 * not move what it names.
 */
pub fn named_at(guest: &Guest, dirfd: u64, name: &[u8]) -> Result<Vec<u8>, i64> {
    let dirfd = super::flags::dirfd(dirfd);
    if name.first() == Some(&b'/') {
        return Ok(name.to_vec());
    }
    if dirfd == AT_FDCWD {
        return Ok(join(&guest.cwd, name));
    }
    match guest.fds.get(dirfd as usize).filter(|f| f.is_open()) {
        Some(fd) if fd.kind == Kind::Dir => Ok(join(&fd.path, name)),
        Some(_) => Err(errno::ENOTDIR),
        None => Err(errno::EBADF),
    }
}

/* `name` under the directory `base`; an absolute `name` is itself. */
pub fn join(base: &[u8], name: &[u8]) -> Vec<u8> {
    if name.first() == Some(&b'/') {
        return name.to_vec();
    }
    [base, b"/", name].concat()
}
