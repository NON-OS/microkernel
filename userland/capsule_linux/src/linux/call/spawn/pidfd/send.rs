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

//! `pidfd_send_signal`: kill, or rt_sigqueueinfo with the caller's siginfo,
//! for the process a pidfd refers to.

use crate::linux::abi::errno;
use crate::linux::call::{kill_from, rt_sigqueueinfo};
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

/// pidfd_send_signal(pidfd, sig, info, flags), its ints read as ints. No
/// flag is served: the thread and group scopes of Linux 6.9 are EINVAL, as a
/// kernel before them answers. A siginfo must carry the call's own signal,
/// and a process that is gone is ESRCH, as Linux answers a reaped one.
pub fn send_signal(guest: &mut Guest, tid: u32, a: [u64; 6]) -> Answer {
    let (sig, info) = (u64::from(a[1] as u32), a[2]);
    if a[3] as u32 != 0 {
        return Answer::value(errno::fail(errno::EINVAL));
    }
    let Some((pid, _)) = super::fd::child(guest, u64::from(a[0] as u32)) else {
        return Answer::value(errno::fail(errno::EBADF));
    };
    if info != 0 {
        let Some(raw) = guest.read(info, 4) else {
            return Answer::value(errno::fail(errno::EFAULT));
        };
        if raw[..] != (sig as u32).to_le_bytes() {
            return Answer::value(errno::fail(errno::EINVAL));
        }
    }
    match (pid, info) {
        (0, _) => Answer::value(errno::fail(errno::ESRCH)),
        (_, 0) => kill_from(guest, tid, u64::from(pid), sig),
        _ => rt_sigqueueinfo(guest, tid, u64::from(pid), sig, info),
    }
}
