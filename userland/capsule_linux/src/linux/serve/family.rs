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

//! Every process this personality hosts: the guest it started and whatever
//! that guest forks. Each keeps its own descriptors, break and cwd. Pipe
//! buffers are the family's, lent to the guest being answered and taken back
//! (`family_lend`): a pipe opened before a fork has its ends in different
//! processes.

use alloc::vec::Vec;
use core::mem;

use nonos_libc::{mk_foreign_reply, ForeignFrame, FOREIGN_NR_DIED};

use super::answer::Answer;
use super::dispatch::answer;
use super::pid_map::frame_in;
use super::pid_ns::PidNs;
use super::pid_out::value_out;
use crate::linux::guest::Guest;

pub struct Family {
    pub(super) guests: Vec<Guest>,
    pub(super) pipes: Vec<Vec<u8>>,
    pub(super) root: u32,
    pub(super) root_code: i32,
    pub(super) ns: PidNs,
}

impl Family {
    pub fn new(mut first: Guest) -> Self {
        let (pipes, root) = (mem::take(&mut first.pipes), first.pid);
        let ns = PidNs::new(first.parent, root);
        Family { guests: alloc::vec![first], pipes, root, root_code: 0, ns }
    }

    pub fn answer(&mut self, frame: &ForeignFrame) {
        if frame.nr == FOREIGN_NR_DIED {
            self.thread_died(frame.pid, frame.arg0 as i32);
            return;
        }
        let Some(i) = self.guests.iter().position(|g| g.owns(frame.pid)) else {
            return;
        };
        let Some(frame) = frame_in(&self.ns, frame) else {
            return;
        };
        self.lend(i);
        let got = answer(&mut self.guests[i], &frame);
        self.take_back(i);
        let g = &mut self.guests[i];
        let born = mem::take(&mut g.forked);
        if let Answer::Reply(value) = got {
            // A caught signal for this thread is delivered in place of the reply.
            let out = value_out(&mut self.ns, frame.nr, value);
            if !super::deliver::maybe_deliver(g, frame.pid, out) {
                let _ = mk_foreign_reply(frame.pid, out);
            }
        }
        self.guests.extend(born);
        self.settle_pipes();
    }

    /// A guest thread ended on a signal. On Linux that ends the thread group,
    /// so the guest exits; reap then kills its other threads and answers any
    /// waiter. The status carries the signal in the shell's 128+signo form.
    fn thread_died(&mut self, pid: u32, code: i32) {
        let Some(g) = self.guests.iter_mut().find(|g| g.owns(pid)) else {
            return;
        };
        g.threads.retain(|t| *t != pid);
        if g.exited.is_none() {
            g.exited = Some(128 + signo_of(code));
        }
        let line =
            alloc::format!("[LINUX] guest thread {pid} ended on a signal; ending the process\n");
        crate::linux::start::say(line.as_bytes());
    }

    /// Done once nothing it hosts is left; the code is the first guest's.
    pub fn done(&self) -> Option<i32> {
        self.guests.is_empty().then_some(self.root_code)
    }
}

/// The kernel names a fatal termination by a code that is not uniform across
/// its exception handlers. Map the ones a guest reaches to a signal number,
/// defaulting to SIGKILL for anything else, for the process's reported status.
fn signo_of(code: i32) -> i32 {
    match code {
        -11 | -12 => 11, // SIGSEGV: page fault, stack, bound, bad segment
        -4 => 4,         // SIGILL: bad opcode, FPU emulation, missing FPU
        -8 => 8,         // SIGFPE: divide, overflow, x87/SSE
        -7 => 7,         // SIGBUS: alignment, virtualisation
        5 => 5,          // SIGTRAP: breakpoint, debug
        c if c > 128 && c < 128 + 64 => c - 128, // terminate_current_with_signal
        _ => 9,          // SIGKILL, and the general-protection sentinel
    }
}
