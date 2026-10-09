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

//! timer_create: a POSIX timer, disarmed. Ids count from 0 per process, as
//! Linux's do. Its signal goes to the process, or to the one thread
//! SIGEV_THREAD_ID names; a NULL sigevent means SIGALRM with the id as its
//! value. Timers on a CPU-time clock are refused by name: the kernel does not
//! report a guest's CPU time to its supervisor.

use super::timer_sigev::read_sigevent;
use crate::linux::abi::errno;
use crate::linux::guest::sigstate::SIGALRM;
use crate::linux::guest::sigtimer::PosixTimer;
use crate::linux::guest::Guest;

const TIMERS_MAX: usize = 1024;

pub fn timer_create(guest: &mut Guest, clock: u64, sevp: u64, out: u64) -> u64 {
    match clock {
        0 | 1 | 7..=9 | 11 => {}
        2 | 3 => {
            let line = b"[LINUX] unserved timer_create: no CPU-time clock\n";
            let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
            return errno::fail(errno::ENOTSUP);
        }
        4..=6 => return errno::fail(errno::ENOTSUP),
        _ => return errno::fail(errno::EINVAL),
    }
    let id = (0..).find(|i| !guest.signals.timers.iter().any(|t| t.id == *i)).unwrap_or(0);
    let mut t = PosixTimer {
        id,
        clock,
        signo: SIGALRM,
        tid: 0,
        value: id as u64,
        due: None,
        interval: 0,
        overrun: 0,
        last_overrun: 0,
        queued: false,
    };
    if sevp != 0 {
        if let Err(e) = read_sigevent(guest, sevp, &mut t) {
            return e;
        }
    }
    if guest.signals.timers.len() >= TIMERS_MAX {
        return errno::fail(errno::EAGAIN);
    }
    if guest.write(out, &id.to_le_bytes()) < 4 {
        return errno::fail(errno::EFAULT);
    }
    guest.signals.timers.push(t);
    errno::ok(0)
}
