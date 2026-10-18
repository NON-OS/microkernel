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

//! Which handle a socket descriptor holds, and whose a thread is.

use super::super::fd::Kind;
use super::super::handle::Guest;

impl Guest {
    /// The net.sockets handle behind `fd`, if it is a socket.
    pub fn socket_handle(&self, fd: u64) -> Option<u32> {
        match self.fds.get(fd as usize) {
            Some(f) if f.kind == Kind::Socket => Some(f.handle),
            _ => None,
        }
    }

    /// True for the guest and for every thread of it, which is what the
    /// serve loop needs: a trap arrives under the thread's own tid.
    pub fn owns(&self, pid: u32) -> bool {
        pid == self.pid || self.threads.contains(&pid)
    }
}
