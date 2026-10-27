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

/* fallocate on a memfd, as tmpfs answers it, which is what a memfd is.
 *
 * mpv's wlshm output sizes each window buffer with posix_fallocate on a
 * memfd and has no ftruncate fallback, and libwayland-cursor falls back
 * only on EINVAL or EOPNOTSUPP: ENODEV here left mpv with no buffer and
 * every cursor theme unloaded. Mode 0 makes it at least `at + len` long;
 * KEEP_SIZE alone has nothing to do; PUNCH_HOLE with KEEP_SIZE zeroes the
 * range of its mapping, inside its size, as the pages would read after. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::calls::{KEEP_SIZE, PUNCH_HOLE};

const CHUNK: usize = 4096;

pub(super) fn memfd(guest: &mut Guest, fd: u64, mode: u64, at: u64, len: u64) -> u64 {
    if mode & !(KEEP_SIZE | PUNCH_HOLE) != 0 {
        return errno::fail(errno::EOPNOTSUPP);
    }
    let Some(entry) = guest.fds.get_mut(fd as usize) else { return errno::fail(errno::EBADF) };
    let Some(end) = at.checked_add(len) else { return errno::fail(errno::EFBIG) };
    if mode == 0 {
        entry.size = entry.size.max(end);
        return errno::ok(0);
    }
    if mode & PUNCH_HOLE == 0 || entry.offset == 0 {
        return errno::ok(0);
    }
    // The mapping's address is kept in the offset (memfd_map.rs).
    let (base, end) = (entry.offset, end.min(entry.size));
    let zeros = [0u8; CHUNK];
    let mut from = at;
    while from < end {
        let n = ((end - from) as usize).min(CHUNK);
        if guest.write(base + from, &zeros[..n]) < n as i64 {
            return errno::fail(errno::EFAULT);
        }
        from += n as u64;
    }
    errno::ok(0)
}
