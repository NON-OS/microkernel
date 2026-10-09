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

//! `writev`, which a C runtime reaches for as often as `write`. The vector
//! is an array of pointer and length pairs in the guest's memory, so it is
//! read out of the guest before any of it is followed.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::iovec::vector;

/// In Linux's order: a descriptor that is not open is EBADF before the
/// array is looked at, even an empty one; the array is checked whole
/// (`iovec`) before any of it is written.
pub fn writev(guest: &mut Guest, fd: u64, iov: u64, count: u64) -> u64 {
    let open = guest.fds.get(fd as usize).is_some_and(|f| f.is_open());
    let pieces = match vector(guest, open, iov, count) {
        Ok(pieces) => pieces,
        Err(e) => return errno::fail(e),
    };
    let mut written = 0u64;
    for (base, len) in pieces {
        if len == 0 {
            continue;
        }
        let result = super::io::write(guest, fd, base, len);
        if (result as i64) < 0 {
            // A failure after a partial write is that partial count.
            return if written == 0 { result } else { errno::ok(written) };
        }
        written += result;
        if result < len {
            break;
        }
    }
    errno::ok(written)
}
