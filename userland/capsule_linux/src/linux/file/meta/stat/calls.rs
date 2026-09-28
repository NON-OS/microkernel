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

/* stat, fstat and newfstatat. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::super::{cache, dev, resolve, store};
use super::super::node::{self};
use super::at::{meta_at, write_out};

pub const AT_SYMLINK_NOFOLLOW: u64 = 0x100;

pub const AT_EMPTY_PATH: u64 = 0x1000;

/* Size and whether it is a directory, for a path already followed. */
pub fn look(full: &[u8]) -> Option<(u64, bool)> {
    if dev::device_of(full).is_some() {
        return Some((0, false));
    }
    if let Some(size) = cache::size(full) {
        return Some((size, false));
    }
    store::stat_full(&resolve::key(full)).ok().map(|(size, is_dir, _, _)| (size, is_dir))
}

pub fn fstat(guest: &mut Guest, fd: u64, out: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize) else {
        return errno::fail(errno::EBADF);
    };
    match node::of_fd(guest, entry) {
        Ok(m) => write_out(guest, out, &m),
        Err(e) => errno::fail(e),
    }
}

pub fn newfstatat(guest: &mut Guest, dirfd: u64, path_ptr: u64, out: u64, flags: u64) -> u64 {
    match meta_at(guest, dirfd, path_ptr, flags) {
        Ok(m) => write_out(guest, out, &m),
        Err(e) => errno::fail(e),
    }
}
