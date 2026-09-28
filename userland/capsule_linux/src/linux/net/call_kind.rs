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

//! What kind of wait a socket call makes: its flags, and whether a
//! blocking one waits until it has moved everything.

use crate::linux::abi::{errno, nr};
use crate::linux::guest::Guest;

use super::flags::{MSG_DONTWAIT, MSG_WAITALL};
use super::{iov, mmsg};

/// The call's flags, where it has them.
pub fn flags(n: u64, a: [u64; 6]) -> u64 {
    match n {
        nr::RECVFROM | nr::SENDTO | nr::RECVMMSG | nr::SENDMMSG | nr::ACCEPT4 => a[3],
        nr::RECVMSG | nr::SENDMSG => a[2],
        _ => 0,
    }
}

/// True when a blocking call waits until it has moved everything it asked
/// for: a send on a stream, a receive with MSG_WAITALL, and recvmmsg
/// without MSG_WAITFORONE.
pub fn wants_all(stream: bool, n: u64, flags: u64) -> bool {
    match n {
        nr::WRITE | nr::WRITEV | nr::SENDTO | nr::SENDMSG => stream,
        nr::RECVFROM | nr::RECVMSG => stream && flags & MSG_WAITALL != 0,
        nr::RECVMMSG => flags & (mmsg::MSG_WAITFORONE | MSG_DONTWAIT) == 0,
        _ => false,
    }
}

/// A receive's answer: the count, or the errno.
pub(super) fn bytes_in(guest: &Guest, id: u32, v: &iov::Iov, done: usize, flags: u64) -> u64 {
    match super::xfer_in::recv(guest, id, v, done, flags) {
        Ok(got) => errno::ok(got.n as u64),
        Err(e) => e,
    }
}

/// The bytes a msghdr's iovecs ask for.
pub(super) fn msg_len(guest: &Guest, msg: u64) -> usize {
    let word = |at: u64| {
        guest.read(at, 8).map_or(0, |b| u64::from_le_bytes(b.try_into().unwrap_or([0; 8])))
    };
    iov::read(guest, word(msg + 16), word(msg + 24)).map_or(0, |v| iov::total(&v))
}
