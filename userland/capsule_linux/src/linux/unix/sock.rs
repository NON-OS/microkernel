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

//! Connecting to the display, and telling a display socket apart.

use crate::linux::abi::errno;
use crate::linux::guest::{Guest, Kind};

use super::path::{is_display, sun_path};

pub fn connect(guest: &mut Guest, fd: u64, at: u64, len: u64) -> u64 {
    let Some(path) = sun_path(guest, at, len) else {
        return errno::fail(errno::EINVAL);
    };
    if !is_display(&path) {
        /*
         * Only the display is served here; every other name is a family
         * socket's (net::unix_calls), which never reaches this call.
         */
        return errno::fail(errno::ECONNREFUSED);
    }
    match guest.fds.get_mut(fd as usize) {
        Some(entry) if entry.kind == Kind::Unix => {
            entry.writable = true;
            errno::ok(0)
        }
        _ => errno::fail(errno::ENOTSOCK),
    }
}

/// True when this descriptor is one of ours rather than a network one.
pub fn is_unix(guest: &Guest, fd: u64) -> bool {
    matches!(guest.fds.get(fd as usize), Some(f) if f.kind == Kind::Unix)
}

pub(super) fn connected(guest: &Guest, fd: u64) -> bool {
    matches!(guest.fds.get(fd as usize), Some(f) if f.kind == Kind::Unix && f.writable)
}
