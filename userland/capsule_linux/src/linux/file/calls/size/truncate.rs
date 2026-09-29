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

/* ftruncate and truncate. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{at, cache, resolve, store, walk};
use super::advice::resize;

pub fn ftruncate(guest: &mut Guest, fd: u64, len: u64) -> u64 {
    if (len as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(entry) = guest.fds.get_mut(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    match entry.kind {
        /*
         * Sizing a memfd records the size and nothing else. The pages appear
         * when the client maps it, because that is when their address is decided.
         */
        Kind::Memfd => {
            entry.size = len;
            errno::ok(0)
        }
        /* Linux answers EINVAL for anything but a regular file open to write. */
        Kind::File if entry.writable && !super::super::super::synth::owns(&entry.path) => {
            let path = entry.path.clone();
            entry.size = len;
            resize(&path, len, true)
        }
        _ => errno::fail(errno::EINVAL),
    }
}

pub fn truncate(guest: &Guest, path: u64, len: u64) -> u64 {
    if (len as i64) < 0 {
        return errno::fail(errno::EINVAL);
    }
    let Some(named) = at::resolve_at(guest, super::super::super::flags::AT_FDCWD, path) else {
        return errno::fail(errno::EFAULT);
    };
    let full = walk::follow(guest, named, true);
    if super::super::super::synth::owns(&full) {
        return errno::fail(errno::EACCES);
    }
    match store::stat(&resolve::key(&full)) {
        _ if cache::held(&full) => {}
        Ok((_, true)) => return errno::fail(errno::EISDIR),
        Ok(_) => {}
        Err(_) => return errno::fail(errno::ENOENT),
    }
    if resolve::key(&full).writable().is_err() {
        return errno::fail(errno::EROFS);
    }
    let kept = cache::held(&full);
    resize(&full, len, kept)
}
