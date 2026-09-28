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

//! One refused call, answered. The ones that can leave a caller parked are
//! taken first; everything else is a plain value.

use nonos_libc::ForeignFrame;

use super::answer::Answer;
use super::table::plain;
use super::waits_sock;
use crate::linux::abi::{nr, nr_path as np};
use crate::linux::call::{clone, exit_thread, futex};
use crate::linux::guest::Guest;

pub fn answer(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    super::tally::call();
    let got = route(guest, frame);
    // An EAGAIN re-arms the descriptor's edge-triggered epoll entries.
    if let Answer::Reply(value) = got {
        crate::linux::file::rearm(guest, frame.nr, frame.args()[0], value);
    }
    got
}

fn route(guest: &mut Guest, frame: &ForeignFrame) -> Answer {
    let a = frame.args();
    match frame.nr {
        nr::CLONE => clone(guest, frame),
        nr::FORK | nr::VFORK => crate::linux::call::fork(guest, frame.pid),
        nr::EXECVE => crate::linux::call::execve(guest, frame.pid, a[0], a[1], a[2]),
        nr::WAIT4 => crate::linux::call::wait4(guest, a[0], a[1], a[2], frame.pid),
        // A thread exiting is not the process exiting.
        nr::EXIT if frame.pid != guest.pid => exit_thread(guest, frame.pid),
        // Never answered: the family ends the process, so it cannot run on.
        nr::EXIT | nr::EXIT_GROUP => {
            let _ = crate::linux::call::exit(guest, a[0]);
            Answer::Park
        }
        nr::RT_SIGRETURN => crate::linux::call::rt_sigreturn(guest, frame.pid),
        nr::FUTEX => futex(guest, frame.pid, a),
        // The caller's own thread, which is not always the process.
        nr::GETTID => Answer::value(u64::from(frame.pid)),
        nr::SET_TID_ADDRESS => crate::linux::call::set_tid_address(guest, frame.pid, a[0]),
        nr::NANOSLEEP => crate::linux::call::nanosleep(guest, frame.pid, a[0]),
        np::CLOCK_NANOSLEEP => {
            crate::linux::call::clock_nanosleep(guest, frame.pid, a[0], a[1], a[2])
        }
        n if waits_sock::takes(guest, n, a[0]) => waits_sock::io(guest, frame.pid, n, a),
        nr::READ | nr::WRITE if super::waits::may_wait(guest, frame.nr, a[0]) => {
            super::waits::io(guest, frame.pid, frame.nr, a)
        }
        nr::EPOLL_PWAIT
        | nr::EPOLL_PWAIT2
        | np::EPOLL_WAIT
        | nr::POLL
        | np::PPOLL
        | np::SELECT
        | np::PSELECT6 => super::waits::timed(guest, frame.pid, frame.nr, a),
        other => Answer::Reply(plain(guest, frame.pid, other, a)),
    }
}
