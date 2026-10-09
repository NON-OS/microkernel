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

//! What a parked call answers when a caught signal ends its wait, as Linux's
//! handle_signal decides it. Pure, so the host proofs hold it.

use crate::linux::abi::errno;

use super::sigstate::SA_RESTART;

/// The calls that answer ERESTARTSYS when a signal ends their wait: read,
/// write, readv, writev, accept, sendto, recvfrom, sendmsg, recvmsg, wait4,
/// fcntl (F_SETLKW, the one that waits), flock, waitid, accept4, recvmmsg
/// and sendmmsg. Every other wait is EINTR, whatever the handler asks.
const RESTARTS: [u64; 16] = [0, 1, 19, 20, 43, 44, 45, 46, 47, 61, 72, 73, 247, 288, 299, 307];
const FUTEX: u64 = 202;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum After {
    /// Run the call again once the handler returns.
    Again,
    /// Return this into the handler's caller.
    Answer(u64),
}

/// `nr` is the parked call, `flags` the handler's sa_flags, `timeout`
/// futex's fourth argument, `moved` what a transfer had already moved, and
/// `timed` whether the wait had a limit of its own (a socket's SO_RCVTIMEO
/// or SO_SNDTIMEO). A transfer that moved something answers that count, as
/// Linux does, and is never run again, which would move it twice. A futex
/// wait with a timeout, and a socket call with one, answer EINTR; so does
/// every call without SA_RESTART.
pub fn after(nr: u64, flags: u64, timeout: u64, moved: u64, timed: bool) -> After {
    if moved != 0 {
        return After::Answer(errno::ok(moved));
    }
    let restartable = RESTARTS.contains(&nr) || (nr == FUTEX && timeout == 0);
    match flags & SA_RESTART != 0 && restartable && !timed {
        true => After::Again,
        false => After::Answer(errno::fail(errno::EINTR)),
    }
}
