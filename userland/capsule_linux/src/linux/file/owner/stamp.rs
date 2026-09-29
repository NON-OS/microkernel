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

/* A time utimensat names, set on the family's record of the file. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::at::resolve_at;
use super::super::resolve::key;
use super::super::times;
use super::chown::AT_SYMLINK_NOFOLLOW;

pub(super) fn stamp(
    guest: &Guest,
    dirfd: u64,
    path: u64,
    flags: u64,
    a: Option<u64>,
    m: Option<u64>,
) -> u64 {
    let full = match path {
        0 => {
            match guest.fds.get(super::super::flags::dirfd(dirfd) as usize).filter(|f| f.is_open())
            {
                Some(f) => f.path.clone(),
                None => return errno::fail(errno::EBADF),
            }
        }
        p => match resolve_at(guest, dirfd, p) {
            Some(named) => {
                super::super::walk::follow(guest, named, flags & AT_SYMLINK_NOFOLLOW == 0)
            }
            None => return errno::fail(errno::EFAULT),
        },
    };
    let now = match super::super::meta::meta_of(guest, full.clone(), true) {
        Ok(now) => now,
        Err(e) => return errno::fail(e),
    };
    if super::super::synth::owns(&full)
        || (!super::super::cache::held(&full) && key(&full).writable().is_err())
    {
        return errno::fail(errno::EROFS);
    }
    let written_ms = super::super::cache::mtime(&full)
        .or_else(|| super::super::store::stat_full(&key(&full)).ok().map(|s| s.2))
        .unwrap_or(0);
    let set = times::Set {
        atime_ms: a.unwrap_or(now.atime_ms),
        mtime_ms: m.unwrap_or(now.mtime_ms),
        written_ms,
    };
    times::set(&full, set);
    errno::ok(0)
}
