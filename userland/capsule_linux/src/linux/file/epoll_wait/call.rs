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

//! epoll_wait's checks, and what it reports written to the guest.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::super::epoll::EVENT_LEN;
use super::collect::collect;

/// The most events one call can ask for, as Linux bounds it.
const MOST: u64 = (i32::MAX as u64) / EVENT_LEN as u64;

/// Report what is ready now, never waiting; `waits` does the waiting.
pub fn epoll_wait(guest: &mut Guest, ep: u64, out: u64, max: u64) -> u64 {
    // maxevents is an int, and one of zero or less is refused.
    if max == 0 || max > MOST {
        return errno::fail(errno::EINVAL);
    }
    /* A closed descriptor is EBADF; an open one that is not an epoll, EINVAL. */
    let Some(list) = guest.fds.get(ep as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    if list.kind != Kind::Epoll {
        return errno::fail(errno::EINVAL);
    }
    let mut watch = list.watch.clone();
    let (blob, hits) = collect(guest, &mut watch, max);
    if !blob.is_empty() && guest.write(out, &blob) < blob.len() as i64 {
        return errno::fail(errno::EFAULT);
    }
    // What was reported, and what was seen, is kept only once it is delivered.
    if let Some(list) = guest.fds.get_mut(ep as usize) {
        list.watch = watch;
    }
    errno::ok(hits)
}
