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

/* The length a file takes, and fadvise64. */

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::super::{cache, resolve, store};

/*
 * Resize the family's copy; with no descriptor to close later, put it in
 * the store now.
 */
pub(crate) fn resize(path: &[u8], len: u64, kept: bool) -> u64 {
    let exists = cache::held(path) || store::stat(&resolve::key(path)).is_ok();
    let done = cache::hold(path, exists).and_then(|()| cache::resize(path, len)).and_then(|()| {
        if kept {
            Ok(())
        } else {
            cache::flush(path, false)
        }
    });
    match done {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}

/* POSIX_FADV_NORMAL to POSIX_FADV_NOREUSE are accepted, and change nothing. */
pub fn fadvise64(guest: &Guest, fd: u64, advice: u64) -> u64 {
    match guest.fds.get(fd as usize).filter(|f| f.is_open()).map(|f| f.kind) {
        None => errno::fail(errno::EBADF),
        Some(Kind::Pipe) => errno::fail(errno::ESPIPE),
        Some(_) if advice > 5 => errno::fail(errno::EINVAL),
        Some(_) => errno::ok(0),
    }
}
