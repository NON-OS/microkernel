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

//! Trying a parked wait again, and what it answers when its time is up.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::call;
use crate::linux::file;
use crate::linux::guest::{Blocked, Guest, Kind};
use crate::linux::net;

/// True for the reads and writes that can wait: a pipe or an eventfd, and
/// a read of a timer.
pub fn may_wait(guest: &Guest, nr: u64, fd: u64) -> bool {
    match guest.fds.get(fd as usize).map(|f| f.kind) {
        Some(Kind::Event | Kind::Pipe) => true,
        Some(Kind::Timer) => nr == nr::READ,
        _ => false,
    }
}

/// The call's answer if it can complete now, None if it would wait.
pub fn attempt(guest: &mut Guest, wait: &Blocked) -> Option<u64> {
    let a = wait.args;
    let again = errno::fail(errno::EAGAIN);
    match wait.nr {
        nr::READ => Some(call::read(guest, a[0], a[1], a[2])).filter(|&v| v != again),
        nr::WRITE => Some(call::write(guest, a[0], a[1], a[2])).filter(|&v| v != again),
        nr::POLL | np::PPOLL => Some(net::poll(guest, a[0], a[1])).filter(|&v| v != 0),
        np::SELECT | np::PSELECT6 => {
            Some(net::select(guest, a[0], [a[1], a[2], a[3]])).filter(|&v| v != 0)
        }
        _ => Some(file::epoll_wait(guest, a[0], a[1], a[2])).filter(|&v| v != 0),
    }
}

/// What a wait answers when its time runs out with nothing ready: zero, and
/// a select's sets emptied, as Linux leaves them.
pub fn expire(guest: &mut Guest, wait: &Blocked) -> u64 {
    let a = wait.args;
    if matches!(wait.nr, np::SELECT | np::PSELECT6) {
        net::select_clear(guest, a[0], [a[1], a[2], a[3]]);
    }
    0
}
