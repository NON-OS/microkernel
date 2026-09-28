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

//! The display's path: the one Unix name this capsule answers itself.

use crate::linux::guest::{Fd, Guest};
use crate::linux::unix;

/// Let go of the family socket and make `fd` the display connection.
pub fn display(guest: &mut Guest, fd: u64, at: u64, len: u64) -> u64 {
    crate::linux::net::close::close(guest, fd);
    if let Some(f) = guest.fds.get_mut(fd as usize) {
        let (cloexec, nonblock) = (f.cloexec, f.nonblock);
        *f = Fd::unix();
        f.cloexec = cloexec;
        f.nonblock = nonblock;
    }
    unix::connect(guest, fd, at, len)
}
