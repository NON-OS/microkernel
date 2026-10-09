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

//! TCP sockets the application has closed but the stack still holds.
//!
//! A close hands the socket back to smoltcp to finish the exchange with the
//! peer; nothing removed it from the socket set afterwards, so every
//! connection ever made stayed there with its buffers. One closed with bytes
//! still unread kept advertising a zero window, and the peer went on probing
//! it for as long as the machine ran.

use alloc::vec::Vec;

use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp;
use spin::Mutex;

static ORPHANS: Mutex<Vec<SocketHandle>> = Mutex::new(Vec::new());

/// Remember a socket the application no longer holds.
pub fn adopt(handle: SocketHandle) {
    ORPHANS.lock().push(handle);
}

/// Forget every orphan. A rebuilt stack has a new socket set, and a handle
/// from the old one would name some other socket in it.
pub fn forget_all() {
    ORPHANS.lock().clear();
}

/// Remove the orphans whose exchange with the peer is over.
pub fn reap(sockets: &mut SocketSet<'static>) {
    ORPHANS.lock().retain(|&h| {
        let done = matches!(
            sockets.get::<tcp::Socket>(h).state(),
            tcp::State::Closed | tcp::State::TimeWait
        );
        if done {
            sockets.remove(h);
        }
        !done
    });
}
