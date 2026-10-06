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

//! Trying a call that may wait, and again once it is parked: a read or write
//! that would block, on a pipe, an eventfd, a timer, the console's input with
//! nothing typed yet, or the console while its launcher's inbox is full; and
//! what a wait answers when its time is up.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::guest::{Blocked, Guest, Kind};
use crate::linux::{call, file, net};

/// True for the reads and writes, plain or vectored, that can wait, and a
/// sendfile to the console.
pub fn may_wait(guest: &Guest, nr: u64, fd: u64) -> bool {
    let reads = matches!(nr, nr::READ | nr::READV);
    if !reads && !matches!(nr, nr::WRITE | nr::WRITEV | np::SENDFILE) {
        return false;
    }
    match guest.fds.get(fd as usize).map(|f| f.kind) {
        Some(Kind::Event | Kind::Pipe) => nr != np::SENDFILE,
        Some(Kind::Timer | Kind::Stdin) => reads,
        Some(Kind::Stdout | Kind::Stderr) => !reads,
        _ => false,
    }
}

/// The call's answer if it can complete now, None if it would wait.
pub fn attempt(guest: &mut Guest, wait: &mut Blocked) -> Option<u64> {
    let a = wait.args;
    let again = |v: u64| Some(v).filter(|&v| v != errno::fail(errno::EAGAIN));
    match wait.nr {
        n if super::waits_sock::takes(guest, n, a[0]) => super::waits_sock::attempt(guest, wait),
        nr::READ => again(call::read(guest, a[0], a[1], a[2])),
        nr::WRITE | nr::WRITEV => super::waits_write::carry_on(guest, wait),
        nr::READV => again(call::readv(guest, a[0], a[1], a[2])),
        np::SENDFILE => again(file::sendfile(guest, a[0], a[1], a[2], a[3])),
        nr::POLL | np::PPOLL => Some(net::poll(guest, a[0], a[1])).filter(|&v| v != 0),
        np::SELECT | np::PSELECT6 => {
            Some(net::select(guest, a[0], [a[1], a[2], a[3]])).filter(|&v| v != 0)
        }
        np::FLOCK | nr::FCNTL => super::waits_lock::retry(guest, wait),
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
