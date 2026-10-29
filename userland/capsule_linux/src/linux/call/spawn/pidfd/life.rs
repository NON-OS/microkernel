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

//! Where a pidfd's process is in its life, what poll reads on the pidfd, and
//! the child a waitid on it waits for. Pure, so the host proofs hold when a
//! pidfd turns readable and which child its waitid reaps.

use crate::linux::abi::errno;

pub const POLLIN: u16 = 0x001;
pub const POLLHUP: u16 = 0x010;
pub const POLLRDNORM: u16 = 0x040;
const WNOHANG: u64 = 1;

/// A pidfd's process: running, ended and not yet waited for, or gone.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Life {
    Running,
    Ended,
    Gone,
}

/// `pid` as the family holds it: running while one of its processes
/// (`live`), ended while a parent holds it as a zombie, and gone after.
pub fn life(pid: u32, live: &[u32], zombies: &[u32]) -> Life {
    match pid {
        0 => Life::Gone,
        p if live.contains(&p) => Life::Running,
        p if zombies.contains(&p) => Life::Ended,
        _ => Life::Gone,
    }
}

/// What a pidfd keeps of its process: the pid, 0 once it is gone, so a later
/// process given the same number is never taken for it; and whether it ended.
pub fn kept(pid: u32, life: Life) -> (u32, bool) {
    match life {
        Life::Running => (pid, false),
        Life::Ended => (pid, true),
        Life::Gone => (0, true),
    }
}

/// poll's bits, as Linux's pidfd_poll gives them: none while the process
/// runs, POLLIN once it has ended, and POLLHUP with it once it is gone.
pub fn bits(pid: u32, ended: bool) -> u16 {
    match (pid, ended) {
        (0, _) => POLLIN | POLLRDNORM | POLLHUP,
        (_, true) => POLLIN | POLLRDNORM,
        _ => 0,
    }
}

/// waitid(P_PIDFD)'s child and options, from what the pidfd keeps and its
/// O_NONBLOCK; EBADF for no pidfd. O_NONBLOCK adds WNOHANG, and unless the
/// caller asked WNOHANG itself, a child still running answers EAGAIN.
pub fn wait_for(kept: Option<(u32, bool)>, options: u64) -> Result<(u32, u64, bool), i64> {
    match kept.ok_or(errno::EBADF)? {
        (pid, true) => Ok((pid, options | WNOHANG, options & WNOHANG == 0)),
        (pid, false) => Ok((pid, options, false)),
    }
}
