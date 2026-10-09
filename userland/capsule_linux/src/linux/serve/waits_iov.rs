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

//! What a waiting write needs to know between tries: whether its descriptor
//! waits for all of it, what it answers after a failure, and its vector.

use alloc::vec::Vec;

use crate::linux::abi::errno;
use crate::linux::guest::{Blocked, Guest, Kind};

/// A blocking pipe, where Linux's writer waits until all of it is in.
pub(super) fn all(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Pipe && !f.nonblock)
}

/// A failure after some bytes went in answers that count, as Linux does.
pub(super) fn so_far(wait: &Blocked, failed: u64) -> u64 {
    if wait.done == 0 {
        failed
    } else {
        errno::ok(wait.done)
    }
}

pub(super) fn vector(guest: &Guest, iov: u64, count: u64) -> Result<Vec<(u64, u64)>, u64> {
    crate::linux::call::iovecs(guest, iov, count).map_err(errno::fail)
}
