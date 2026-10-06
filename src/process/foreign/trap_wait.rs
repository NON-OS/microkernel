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
        Answer::Value(value) => {
            /*
             * MkPeerTls sets the thread pointer in the control block, and a
             * switch is what writes the control block's base to the CPU. A
             * guest answered before it switched out, as on several CPUs it
             * can be while still in wait_raw, never passes one, so a base
             * set while it was parked stayed off the CPU: a program answered
             * arch_prctl(ARCH_SET_FS) that way ran on FS 0 after its exec and
             * faulted at -4 on its first TLS read. Every value return installs
             * the control block's base, switch or not.
             */
            let base = crate::process::with_process(pid, |p| p.get_tls_base()).unwrap_or(0);
            crate::arch::context::set_user_tls(base);
            value
        }
        Answer::Execed => {
            super::signal_fpu::forget(pid);
            super::exec_enter::enter(pid)
        }
        Answer::Deliver(ctx) => super::signal_enter::deliver(pid, ctx),
        Answer::Sigreturn(ctx) => super::signal_enter::sigreturn(pid, ctx),
    }
}
