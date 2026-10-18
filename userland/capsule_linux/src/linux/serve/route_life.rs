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

//! Every guest call starts here. One outside the program's declared calls is
//! refused at once, never parked or routed. Then the lifecycle and signal
//! calls that can leave their caller parked (exit, a new process, a wait, a
//! signal only the family can deliver) are answered, before `dispatch`.

use nonos_libc::ForeignFrame;

use super::answer::Answer;
use crate::linux::abi::errno;
use crate::linux::abi::{nr, nr_sig as ns};
use crate::linux::call;
use crate::linux::guest::Guest;

/// clone's CLONE_THREAD: without it, clone makes a process.
const CLONE_THREAD: u64 = 0x10000;

/// One trap answered: refused, here, or by dispatch.
pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let refused = crate::linux::app::outside(guest, frame.nr).map(Answer::value);
    match refused.or_else(|| first(guest, frame)) {
        Some(got) => {
            super::tally::call();
            got
        }
        None => super::dispatch::answer(guest, frame),
    }
}

fn first(guest: &mut Guest, frame: &ForeignFrame) -> Option<Answer> {
    let (a, tid) = (frame.args(), frame.pid);
    Some(match frame.nr {
        nr::EXIT => call::exit_one(guest, tid, a[0]),
        nr::EXIT_GROUP => {
            let _ = call::exit(guest, a[0]);
            Answer::Park
        }
        nr::CLONE if a[0] & CLONE_THREAD == 0 => call::clone_process(guest, tid, a),
        nr::VFORK => call::vfork(guest, tid),
        nr::WAIT4 => call::wait4_usage(guest, a[0], a[1], a[2], a[3], tid),
        ns::WAITID => call::waitid(guest, tid, a),
        ns::KILL => call::kill_from(guest, tid, a[0], a[1]),
        ns::TKILL => call::tgkill_from(guest, tid, 0, a[0], a[1]),
        ns::TGKILL if (a[0] as i64) <= 0 => Answer::value(errno::fail(errno::EINVAL)),
        ns::TGKILL => call::tgkill_from(guest, tid, a[0], a[1], a[2]),
        ns::RT_SIGQUEUEINFO => call::rt_sigqueueinfo(guest, tid, a[0], a[1], a[2]),
        ns::RT_TGSIGQUEUEINFO => call::rt_tgsigqueueinfo(guest, tid, a[0], a[1], a[2], a[3]),
        ns::PAUSE => call::pause(guest, tid),
        ns::SIGNALFD4 => Answer::value(call::signalfd4(guest, a[0], a[1], a[2], a[3])),
        ns::SIGNALFD => Answer::value(call::signalfd4(guest, a[0], a[1], a[2], 0)),
        nr::READ if signalfd(guest, a[0]) => call::signalfd_read(guest, tid, a[0], a[1], a[2]),
        ns::RT_SIGSUSPEND => call::rt_sigsuspend(guest, tid, a[0], a[1]),
        ns::RT_SIGTIMEDWAIT => call::rt_sigtimedwait(guest, tid, a[0], a[1], a[2], a[3]),
        _ => return None,
    })
}

fn signalfd(guest: &Guest, fd: u64) -> bool {
    guest.fds.get(fd as usize).is_some_and(|f| f.kind == crate::linux::guest::Kind::Signal)
}
