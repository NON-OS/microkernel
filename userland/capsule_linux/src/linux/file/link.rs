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

//! `symlinkat` and `linkat`; the plain forms are these at AT_FDCWD.
//!
//! A symbolic link joins the family's link table, where the image's own links
//! are, and only where the guest may write. A hard link is the same bytes
//! under a second name, copied: the store has no inodes to share, and a copy
//! keeps what programs rely on, that removing the old name leaves the new.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::at::resolve_at;
use super::path::read_path;
use super::resolve::key;
use super::{meta::stat, store_read, store_write};

/// Largest file a hard link copies; the same bound an exec image has.
const MAX_LINKED: u32 = 64 << 20;

pub fn symlinkat(guest: &Guest, target: u64, dirfd: u64, path: u64) -> u64 {
    let (Some(to), Some(at)) = (read_path(guest, target), resolve_at(guest, dirfd, path)) else {
        return errno::fail(errno::EFAULT);
    };
    if let Err(e) = free_and_writable(guest, &at) {
        return errno::fail(e);
    }
    match guest.links.add(at, to) {
        true => errno::ok(0),
        false => errno::fail(errno::EEXIST),
    }
}

pub fn linkat(guest: &Guest, olddir: u64, old: u64, newdir: u64, new: u64) -> u64 {
    let (Some(from), Some(at)) = (resolve_at(guest, olddir, old), resolve_at(guest, newdir, new))
    else {
        return errno::fail(errno::EFAULT);
    };
    let from = guest.links.follow(from, true);
    if let Err(e) = free_and_writable(guest, &at) {
        return errno::fail(e);
    }
    let Ok(bytes) = store_read(&key(&from), MAX_LINKED) else {
        return errno::fail(errno::ENOENT);
    };
    match store_write(&key(&at), &bytes) {
        Ok(()) => errno::ok(0),
        Err(_) => errno::fail(errno::EIO),
    }
}

// A new name must not exist as a file or a link, and must be somewhere the
// guest may write: the shared tree is read-only to it.
fn free_and_writable(guest: &Guest, at: &[u8]) -> Result<(), i64> {
    if stat::look(at).is_some() || guest.links.target(at).is_some() {
        return Err(errno::EEXIST);
    }
    key(at).writable().map_err(|_| errno::EROFS)
}
