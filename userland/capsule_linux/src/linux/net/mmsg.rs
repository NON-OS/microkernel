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

//! `sendmmsg` and `recvmmsg`: several messages in one call. A blocking
//! recvmmsg waits until all `vlen` have come, unless MSG_WAITFORONE; the
//! wait keeps its count between tries (`waits_sock`), so each try starts at
//! message `skip` and answers how many it moved.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::flags::MSG_DONTWAIT;
use super::mmsg_each::each;
use super::policy::refuse;

/// Linux's UIO_MAXIOV caps vlen.
pub const MOST: u64 = 1024;
pub const MSG_WAITFORONE: u64 = 0x10000;

/// `a` is sendmmsg's arguments: fd, vector, vlen, flags.
pub fn sendmmsg(guest: &mut Guest, a: [u64; 6], skip: usize) -> u64 {
    each(guest, a, skip, |g, at, first| {
        let f = if first { a[3] } else { a[3] | MSG_DONTWAIT };
        super::msg::sendmsg(g, a[0], at, f, 0)
    })
}

/// `a` is recvmmsg's arguments: fd, vector, vlen, flags, timeout.
pub fn recvmmsg(guest: &mut Guest, a: [u64; 6], skip: usize) -> u64 {
    if a[4] != 0 {
        return refuse("recvmmsg timeout: SO_RCVTIMEO bounds the wait instead", errno::EINVAL);
    }
    let flags = a[3] & !MSG_WAITFORONE;
    each(guest, a, skip, |g, at, first| {
        let f = if first { flags } else { flags | MSG_DONTWAIT };
        super::msg_recv::recvmsg(g, a[0], at, f, 0)
    })
}
