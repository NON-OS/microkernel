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

/* Opening a name once it is walked: links, made trees, the store. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::flags::{writes, O_CREAT, O_DIRECTORY, O_EXCL, O_NOFOLLOW};
use super::super::{cache, dev, dir, regular, resolve, store, synth_ops, walk};

/* Open the path the guest named, once made absolute. */
pub fn open_named(guest: &mut Guest, named: Vec<u8>, flags: u64, mode: u64) -> u64 {
    /* O_NOFOLLOW refuses a link in the last place, with ELOOP, as Linux does. */
    if flags & O_NOFOLLOW != 0 && super::super::meta::is_link(guest, &named) {
        return errno::fail(errno::ELOOP);
    }
    let full = walk::follow(guest, named, true);
    /* /dev/null and its kin are descriptors this capsule answers itself. */
    if dev::device_of(&full).is_some() {
        return dev::open_path(guest, &full, flags);
    }
    if let Some(got) = synth_ops::open(guest, &full, flags) {
        return got;
    }
    let found = match cache::size(&full) {
        Some(size) => Some((size, false)),
        None => store::stat(&resolve::key(&full)).ok(),
    };
    match found {
        Some(_) if flags & O_CREAT != 0 && flags & O_EXCL != 0 => errno::fail(errno::EEXIST),
        Some((_, true)) if writes(flags) => errno::fail(errno::EISDIR),
        Some((_, true)) => dir::open(guest, full),
        Some((_, false)) if flags & O_DIRECTORY != 0 => errno::fail(errno::ENOTDIR),
        Some((size, false)) => regular::open(guest, full, size, flags),
        None if flags & O_CREAT != 0 => regular::create(guest, full, flags, mode),
        None => errno::fail(errno::ENOENT),
    }
}
