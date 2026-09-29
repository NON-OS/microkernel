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

//! A guest asleep inside the syscall it made.

use super::trap_table::{take_answer, Answer};

pub(super) fn wait_for_answer(pid: u32) -> u64 {
    settle(pid, wait_raw(pid))
}

/// Sleep until the supervisor answers, and take the answer as it is.
pub(super) fn wait_raw(pid: u32) -> Answer {
    loop {
        if let Some(answer) = take_answer(pid) {
            super::trap_frame::drop_frame(pid);
            return answer;
        }
        let token = crate::sched::wake_token(pid);
        if let Some(answer) = take_answer(pid) {
            super::trap_frame::drop_frame(pid);
            return answer;
        }
        crate::sched::sleep_until_unless_woken(pid, u64::MAX, token);
        crate::sched::yield_now();
    }
}

/// A value returns; exec and a signal leave by a context of their own.
fn settle(pid: u32, answer: Answer) -> u64 {
    match answer {
        Answer::Value(value) => value,
        Answer::Execed => {
            super::signal_fpu::forget(pid);
            super::exec_enter::enter(pid)
        }
        Answer::Deliver(ctx) => {
            /* A mark is spent only by a handler entered, not by any answer. */
            super::interrupt::forget(pid);
            super::signal_enter::deliver(pid, ctx)
        }
        Answer::Sigreturn(ctx) => super::signal_enter::sigreturn(pid, ctx),
    }
}
