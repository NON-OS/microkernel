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

//! Which calls `waits_sock` takes, and which of its waits need a tick.

use crate::linux::abi::{nr, nr_path as np};
use crate::linux::guest::{Blocked, Guest, Kind};
use crate::linux::net;

/// True for a call on a socket descriptor that may have to wait.
pub fn takes(guest: &Guest, n: u64, fd: u64) -> bool {
    matches!(
        n,
        nr::READ
            | nr::WRITE
            | np::READV
            | nr::WRITEV
            | nr::ACCEPT
            | nr::ACCEPT4
            | nr::CONNECT
            | nr::SENDTO
            | nr::RECVFROM
            | nr::SENDMSG
            | nr::RECVMSG
            | nr::SENDMMSG
            | nr::RECVMMSG
    ) && guest.fds.get(fd as usize).is_some_and(|f| f.kind == Kind::Socket)
}

/// True when a parked call waits on a stream net.sockets holds, which
/// nothing but a look tells the family has changed.
pub fn ticks(guest: &Guest, wait: &Blocked) -> bool {
    takes(guest, wait.nr, wait.args[0])
        && net::sock_id(guest, wait.args[0]).is_some_and(net::outside)
}
