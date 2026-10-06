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

/* Opening a regular file of the store or of the family's copies. */

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Fd, Guest};

use super::super::flags::{wants_read, writes, O_APPEND, O_TRUNC};
use super::super::{cache, desc, resolve, slot, store};

pub fn open(guest: &mut Guest, path: Vec<u8>, size: u64, flags: u64) -> u64 {
    let writing = writes(flags);
    if writing && resolve::key(&path).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    let truncating = flags & O_TRUNC != 0 && writing;
    let stream = match wants_read(flags) && !cache::held(&path) {
        true => match store::open(&resolve::key(&path)) {
            Ok(s) => Some(s),
            Err(_) => return errno::fail(errno::EACCES),
        },
        false => None,
    };
    /* The store has the file, `size` bytes of it, which close replaces. */
    if truncating {
        if let Err(e) = cache::hold_emptied(&path, size) {
            return errno::fail(e);
        }
    }
    let size = if truncating { 0 } else { cache::size(&path).unwrap_or(size) };
    install(guest, Fd::file(path, size, stream, writing), flags)
}

pub(super) fn install(guest: &mut Guest, mut fd: Fd, flags: u64) -> u64 {
    fd.handle = desc::fresh(flags & O_APPEND != 0, wants_read(flags));
    match slot::install(guest, fd) {
        Some(n) => errno::ok(n),
        None => errno::fail(errno::EMFILE),
    }
}
