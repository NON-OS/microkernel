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

//! Entering a handler: the stack it disarms, and what an interrupted wait answers.

use super::stack::{SET, SIZE, SP};
use crate::linux::abi::errno::{fail, ok, EINTR};
use crate::linux::guest::sigalt::stack::SS_AUTODISARM;
use crate::linux::guest::sigalt::{change, entered, NONE};
use crate::linux::guest::sigrestart::{after, After};
use crate::linux::guest::sigstate::SA_RESTART;

#[test]
fn an_interrupted_wait_answers_as_linux_handle_signal_does() {
    const R: u64 = SA_RESTART;
    let again = After::Again;
    let eintr = After::Answer(fail(EINTR));
    // (nr, sa_flags, futex timeout, moved, timed, answer)
    let table: [(u64, u64, u64, u64, bool, After); 15] = [
        (0, R, 0, 0, false, again),
        (0, 0, 0, 0, false, eintr),
        (61, R, 0, 0, false, again),
        (247, R, 0, 0, false, again),
        // The lock waits and the multi-message socket calls restart too.
        (72, R, 0, 0, false, again),
        (73, R, 0, 0, false, again),
        (299, R, 0, 0, false, again),
        (202, R, 0, 0, false, again),
        (202, R, 0x7000, 0, false, eintr),
        // poll, epoll_wait, nanosleep and rt_sigsuspend never restart.
        (7, R, 0, 0, false, eintr),
        (232, R, 0, 0, false, eintr),
        (35, R, 0, 0, false, eintr),
        (130, R, 0, 0, false, eintr),
        // A socket wait with SO_RCVTIMEO is EINTR, SA_RESTART or not.
        (45, R, 0, 0, true, eintr),
        // A write that moved something answers that, and never runs again.
        (1, R, 0, 4096, false, After::Answer(ok(4096))),
    ];
    for (nr, flags, timeout, moved, timed, want) in table {
        assert_eq!(after(nr, flags, timeout, moved, timed), want, "nr {nr}");
    }
}

#[test]
fn entering_a_handler_disarms_only_an_autodisarm_stack() {
    let armed = [SP, SS_AUTODISARM, SIZE];
    assert_eq!(entered(armed), NONE);
    assert_eq!(entered(SET), SET);
    // The frame records it armed, and its return puts it back.
    assert_eq!(change(entered(armed), SP + 64, armed), Ok(armed));
}
