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

//! alarm, setitimer and getitimer. ITIMER_REAL counts on the monotonic clock
//! and raises SIGALRM at the process when it runs out; exec keeps it, fork
//! does not. ITIMER_VIRTUAL and ITIMER_PROF count the process's CPU time,
//! which the kernel does not report to a supervisor: arming one is refused by
//! name, and reading one reports it disarmed, which it always is.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::signal_itv::{put, read_val};
use super::signal_real::{arm, remaining};

const ITIMER_REAL: u64 = 0;
const ITIMER_PROF: u64 = 2;

/// alarm(seconds): a one-shot ITIMER_REAL, answering the whole seconds the
/// last one had left, rounded as Linux rounds them.
pub fn alarm(guest: &mut Guest, secs: u64) -> u64 {
    let (was, _) = arm(guest, secs.min(u64::from(u32::MAX)).saturating_mul(1000), 0);
    let (whole, ms) = (was / 1000, was % 1000);
    errno::ok(whole + u64::from((whole == 0 && ms > 0) || ms >= 500))
}

pub fn setitimer(guest: &mut Guest, which: u64, new: u64, old: u64) -> u64 {
    if which > ITIMER_PROF {
        return errno::fail(errno::EINVAL);
    }
    let (interval, value) = match new {
        0 => (0, 0),
        at => match read_val(guest, at) {
            Ok(v) => v,
            Err(e) => return e,
        },
    };
    if which != ITIMER_REAL {
        if value == 0 {
            return put(guest, old, 0, 0);
        }
        let _ = nonos_libc::mk_debug(UNSERVED.as_ptr(), UNSERVED.len());
        return errno::fail(errno::ENOSYS);
    }
    let (was, every) = arm(guest, value, interval);
    put(guest, old, every, was)
}

const UNSERVED: &[u8] = b"[LINUX] unserved setitimer: no CPU-time clock for VIRTUAL or PROF\n";

pub fn getitimer(guest: &mut Guest, which: u64, out: u64) -> u64 {
    if which > ITIMER_PROF {
        return errno::fail(errno::EINVAL);
    }
    let (left, every) = match which {
        ITIMER_REAL => remaining(guest),
        _ => (0, 0),
    };
    put(guest, out, every, left)
}
