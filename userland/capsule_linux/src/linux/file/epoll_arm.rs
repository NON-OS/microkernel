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

//! Re-arming an edge-triggered epoll entry.
//!
//! A program using EPOLLET reads or writes until a call answers EAGAIN and
//! only then waits. That answer is where the next rise must be reported
//! again, even if the family never looked while the readiness was low.

use crate::linux::abi::{errno, nr};
use crate::linux::guest::{Guest, Kind};

const EPOLLIN: u32 = 0x001;
const EPOLLOUT: u32 = 0x004;

/// The calls that wait for a descriptor to become readable, and writable.
const READS: [u64; 7] =
    [nr::READ, nr::READV, nr::PREAD64, nr::RECVFROM, nr::RECVMSG, nr::ACCEPT, nr::ACCEPT4];
const WRITES: [u64; 5] = [nr::WRITE, nr::WRITEV, nr::PWRITE64, nr::SENDTO, nr::SENDMSG];

/// After a call on `fd` answered `value`, forget that its readiness was seen.
pub fn rearm(guest: &mut Guest, number: u64, fd: u64, value: u64) {
    let again = value == errno::fail(errno::EAGAIN);
    let bits = if again && READS.contains(&number) {
        EPOLLIN
    } else if again && WRITES.contains(&number) {
        EPOLLOUT
    } else if number == nr::CONNECT && value == errno::fail(errno::EINPROGRESS) {
        EPOLLOUT
    } else {
        return;
    };
    for list in guest.fds.iter_mut().filter(|f| f.kind == Kind::Epoll) {
        for w in list.watch.iter_mut().filter(|w| w.fd == fd) {
            w.fired &= !bits;
        }
    }
}
