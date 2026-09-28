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
use super::path::read_path;
use super::resolve::visible;

pub fn resolve_at(guest: &Guest, dirfd: u64, path: u64) -> Option<Vec<u8>> {
    let name = read_path(guest, path)?;
    let full = named_at(guest, dirfd, &name).ok()?;
    /* The *at calls act on the name, so its own last component is not followed. */
    Some(super::walk::follow(guest, full, false))
}

/*
 * The absolute name `name` gives against `dirfd`, nothing followed yet.
 * A directory descriptor keeps the path it was opened at, so a chdir made
 * since does not move what it names.
 */
pub fn named_at(guest: &Guest, dirfd: u64, name: &[u8]) -> Result<Vec<u8>, i64> {
    let dirfd = super::flags::dirfd(dirfd);
    if name.first() == Some(&b'/') {
        return Ok(visible(b"/", name));
    }
    if dirfd == AT_FDCWD {
        return Ok(visible(&guest.cwd, name));
    }
    match guest.fds.get(dirfd as usize).filter(|f| f.is_open()) {
        Some(fd) if fd.kind == Kind::Dir => Ok(visible(&fd.path, name)),
        Some(_) => Err(errno::ENOTDIR),
        None => Err(errno::EBADF),
    }
}
