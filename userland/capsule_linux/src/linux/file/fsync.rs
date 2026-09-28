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
//! Getting a descriptor's buffered bytes onto the store.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

/*
 * fsync and fdatasync: the store keeps no metadata apart from the bytes,
 * so the two are one.
 */
pub fn fsync(guest: &Guest, fd: u64) -> u64 {
    let Some(entry) = guest.fds.get(fd as usize).filter(|f| f.is_open()) else {
        return errno::fail(errno::EBADF);
    };
    /* Linux answers EINVAL for what cannot be synced: a pipe, a socket. */
    if !matches!(entry.kind, Kind::File | Kind::Dir) {
        return errno::fail(errno::EINVAL);
    }
    if entry.kind == Kind::Dir || super::synth::owns(&entry.path) {
        return errno::ok(0);
    }
    match super::cache::flush(&entry.path, true) {
        Ok(()) => errno::ok(0),
        Err(e) => errno::fail(e),
    }
}
