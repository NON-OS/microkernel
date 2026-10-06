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


//! `sendmsg` and `recvmsg`, which is how libwayland actually talks.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;
use crate::linux::wayland;

use super::msg::read_msghdr;

/// The most descriptors passed and not yet taken: what a guest may hold.
const MOST_FDS: usize = 256;
const ETOOMANYREFS: i64 = 109;
use super::sock::connected;

pub fn sendmsg(guest: &mut Guest, fd: u64, at: u64) -> u64 {
    if !connected(guest, fd) {
        return errno::fail(errno::ENOTCONN);
    }
    let Some(msg) = read_msghdr(guest, at) else {
        return errno::fail(errno::EFAULT);
    };
    /*
     * Descriptors wait here until a request takes them. A client that keeps
     * passing ones nothing takes would grow the list for good; Linux refuses
     * more in flight than a process may hold with ETOOMANYREFS.
     */
    if guest.display.fds.len() + msg.fds.len() > MOST_FDS {
        return errno::fail(ETOOMANYREFS);
    }
    /*
     * As much as the connection has room for (`Conn::room`), the descriptors
     * with the first byte, as a stream socket's sendmsg sends them; nothing
     * at all, EAGAIN, with no room.
     */
    let take = msg.bytes.len().min(guest.display.room());
    if take == 0 && !msg.bytes.is_empty() {
        return errno::fail(errno::EAGAIN);
    }
    guest.display.fds.extend_from_slice(&msg.fds);
    guest.display.to_server.extend_from_slice(&msg.bytes[..take]);
    wayland::serve(guest);
    errno::ok(take as u64)
}
