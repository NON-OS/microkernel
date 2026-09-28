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

/* openat. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::flags::{writes, O_CLOEXEC, O_CREAT, O_DIRECTORY, O_EXCL};
use super::super::{cache, dev, dir, path, regular, resolve, store};
use super::mark::{base_of, mark};

pub fn openat(guest: &mut Guest, dirfd: u64, path_ptr: u64, flags: u64) -> u64 {
    let Some(name) = path::read_path(guest, path_ptr) else {
        return errno::fail(errno::EFAULT);
    };
    let base = match base_of(guest, dirfd) {
        Ok(base) => base,
        Err(e) => return e,
    };
    let full = guest.links.follow(resolve::visible(&base, &name), true);
    /* A file the family is making is there before the store holds it. */
    let found = match cache::size(&full) {
        Some(size) => Some((size, false)),
        None => store::stat(&resolve::key(&full)).ok(),
    };
    let got = match found {
        _ if dev::device_of(&full).is_some() => dev::open_path(guest, &full, flags),
        Some(_) if flags & O_CREAT != 0 && flags & O_EXCL != 0 => errno::fail(errno::EEXIST),
        Some((_, true)) if writes(flags) => errno::fail(errno::EISDIR),
        Some((_, true)) => dir::open(guest, full),
        Some((_, false)) if flags & O_DIRECTORY != 0 => errno::fail(errno::ENOTDIR),
        Some((size, false)) => regular::open(guest, full, size, flags),
        None if flags & O_CREAT != 0 => regular::create(guest, full, flags),
        None => errno::fail(errno::ENOENT),
    };
    mark(guest, got, flags & O_CLOEXEC != 0);
    got
}
