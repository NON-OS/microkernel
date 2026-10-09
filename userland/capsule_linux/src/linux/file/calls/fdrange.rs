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

/*
 * close_range: close, or mark close-on-exec, every descriptor from `first`
 * to `last`.
 */

use crate::linux::abi::errno;
use crate::linux::call;
use crate::linux::guest::Guest;

/*
 * Linux's CLOSE_RANGE_UNSHARE: a guest's table is never shared with
 * another process's, so there is nothing to unshare.
 */
const UNSHARE: u64 = 1 << 1;
const CLOEXEC: u64 = 1 << 2;

pub fn close_range(guest: &mut Guest, first: u64, last: u64, flags: u64) -> u64 {
    if flags & !(UNSHARE | CLOEXEC) != 0 || first > last {
        return errno::fail(errno::EINVAL);
    }
    let end = (last as usize).min(guest.fds.len().saturating_sub(1));
    for fd in first as usize..=end {
        if !guest.fds.get(fd).is_some_and(|f| f.is_open()) {
            continue;
        }
        match flags & CLOEXEC {
            0 => {
                let _ = call::close(guest, fd as u64);
            }
            _ => guest.fds[fd].cloexec = true,
        }
    }
    errno::ok(0)
}
