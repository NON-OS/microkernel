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
//! it for as long as the machine ran. One whose peer never closes waits for
//! good, so how long and how many are bounded too (orphan_rule.rs).

use alloc::vec::Vec;

use nonos_libc::mk_uptime_ms;
use smoltcp::iface::{SocketHandle, SocketSet};
use smoltcp::socket::tcp;
use spin::Mutex;

use super::orphan_rule::goes;

/// Each orphan, oldest first, with when it was adopted.
static ORPHANS: Mutex<Vec<(SocketHandle, i64)>> = Mutex::new(Vec::new());

/// Remember a socket the application no longer holds.
pub fn adopt(handle: SocketHandle) {
    ORPHANS.lock().push((handle, mk_uptime_ms()));
}

/// Forget every orphan. A rebuilt stack has a new socket set, and a handle
/// from the old one would name some other socket in it.
pub fn forget_all() {
    ORPHANS.lock().clear();
}

/// Remove the orphans whose exchange with the peer is over, or that have
/// waited as long as Linux would, or the oldest past the most held; one
/// still open is reset first.
pub fn reap(sockets: &mut SocketSet<'static>) {
    let now = mk_uptime_ms();
    let mut orphans = ORPHANS.lock();
    let count = orphans.len();
    let mut index = 0;
    orphans.retain(|&(h, at)| {
        let sock = sockets.get_mut::<tcp::Socket>(h);
        let done = matches!(sock.state(), tcp::State::Closed | tcp::State::TimeWait);
        let gone = goes(done, now.wrapping_sub(at), index, count);
        index += 1;
        if gone && !done {
            sock.abort();
        }
        if gone {
            sockets.remove(h);
        }
        !gone
    });
}
