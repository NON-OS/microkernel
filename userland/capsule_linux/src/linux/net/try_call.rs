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

//! One try at a socket call that may wait. The serve loop's `waits_sock`
//! parks a blocking one that cannot finish and tries it again; `done` is
//! how much of it an earlier try moved (bytes, or messages for the mmsg
//! calls), and a try goes on from there.

use alloc::vec;

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::guest::Guest;

use super::call_kind::{bytes_in, msg_len};
use super::{iov, mmsg};

/// The answer, and the whole amount the call asks to move.
pub fn try_call(guest: &mut Guest, n: u64, a: [u64; 6], done: usize) -> (u64, usize) {
    let fd = a[0];
    let Ok(id) = super::fd::sock_of(guest, fd) else {
        return (errno::fail(errno::EBADF), 0);
    };
    let one = |buf: u64, len: u64| vec![(buf, len)];
    match n {
        nr::READ => (bytes_in(guest, id, &one(a[1], a[2]), done, 0), a[2] as usize),
        nr::WRITE => {
            (super::xfer_out::send(guest, id, &one(a[1], a[2]), done, 0, None), a[2] as usize)
        }
        np::READV | nr::WRITEV => match iov::read(guest, a[1], a[2]) {
            Ok(v) if n == nr::WRITEV => {
                (super::xfer_out::send(guest, id, &v, done, 0, None), iov::total(&v))
            }
            Ok(v) => (bytes_in(guest, id, &v, done, 0), iov::total(&v)),
            Err(e) => (e, 0),
        },
        nr::RECVFROM if done == 0 => {
            (super::recvfrom(guest, fd, a[1], a[2], a[3], a[4], a[5]), a[2] as usize)
        }
        nr::RECVFROM => (bytes_in(guest, id, &one(a[1], a[2]), done, a[3]), a[2] as usize),
        nr::SENDTO if done == 0 => {
            (super::sendto(guest, fd, a[1], a[2], a[3], a[4], a[5]), a[2] as usize)
        }
        nr::SENDTO => {
            (super::xfer_out::send(guest, id, &one(a[1], a[2]), done, a[3], None), a[2] as usize)
        }
        nr::SENDMSG => (super::msg::sendmsg(guest, fd, a[1], a[2], done), msg_len(guest, a[1])),
        nr::RECVMSG => {
            (super::msg_recv::recvmsg(guest, fd, a[1], a[2], done), msg_len(guest, a[1]))
        }
        nr::SENDMMSG => (mmsg::sendmmsg(guest, a, done), a[2].min(mmsg::MOST) as usize),
        nr::RECVMMSG => (mmsg::recvmmsg(guest, a, done), a[2].min(mmsg::MOST) as usize),
        nr::ACCEPT => (super::accept::accept4(guest, fd, a[1], a[2], 0), 0),
        nr::ACCEPT4 => (super::accept::accept4(guest, fd, a[1], a[2], a[3]), 0),
        nr::CONNECT => (super::connect(guest, fd, a[1], a[2]), 0),
        _ => (errno::fail(errno::ENOSYS), 0),
    }
}
