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

//! rt_sigprocmask: each thread's mask of the signals it holds back, which
//! `serve::deliver` reads before a signal is taken.

use super::signal::SIGSET_LEN;
use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const SIG_BLOCK: u64 = 0;
const SIG_UNBLOCK: u64 = 1;
const SIG_SETMASK: u64 = 2;

/// The calling thread's mask: added to, taken from or replaced. The old mask
/// is written after the change is made, as Linux orders it.
pub fn rt_sigprocmask(guest: &mut Guest, tid: u32, how: u64, set: u64, old: u64, size: u64) -> u64 {
    if size != SIGSET_LEN {
        return errno::fail(errno::EINVAL);
    }
    let was = guest.signals.blocked(tid);
    if set != 0 {
        let Some(raw) = guest.read(set, 8) else {
            return errno::fail(errno::EFAULT);
        };
        let new = u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8]));
        let mask = match how {
            SIG_BLOCK => was | new,
            SIG_UNBLOCK => was & !new,
            SIG_SETMASK => new,
            _ => return errno::fail(errno::EINVAL),
        };
        guest.signals.set_blocked(tid, mask);
    }
    if old != 0 && guest.write(old, &was.to_le_bytes()) < 8 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
