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

//! Signal dispositions, masks and stacks, and the timers that end in a
//! signal: the calls of that family that answer at once.

use crate::linux::abi::{nr, nr_sig as ns};
use crate::linux::call;
use crate::linux::guest::Guest;

pub fn sig_ops(guest: &mut Guest, tid: u32, nr: u64, a: [u64; 6]) -> Option<u64> {
    Some(match nr {
        nr::RT_SIGACTION => call::rt_sigaction(guest, a[0], a[1], a[2], a[3]),
        nr::RT_SIGPROCMASK => call::rt_sigprocmask(guest, tid, a[0], a[1], a[2], a[3]),
        nr::SIGALTSTACK => call::sigaltstack(guest, tid, a[0], a[1]),
        ns::ALARM => call::alarm(guest, a[0]),
        ns::SETITIMER => call::setitimer(guest, a[0], a[1], a[2]),
        ns::GETITIMER => call::getitimer(guest, a[0], a[1]),
        _ => return None,
    })
}
