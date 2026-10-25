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

//! A guest thread the kernel ended, and the signal its process reports.

use super::family::Family;

impl Family {
    /// A guest thread ended on a signal. On Linux that ends the thread group,
    /// so the guest exits; reap then kills its other threads and answers any
    /// waiter. The status is Linux's wait status: the signal's number.
    pub(super) fn thread_died(&mut self, pid: u32, code: i32) {
        let Some(g) = self.guests.iter_mut().find(|g| g.owns(pid)) else {
            return;
        };
        g.threads.retain(|t| *t != pid);
        crate::linux::call::killed(g, signo_of(code) as u8);
        crate::linux::recent_say::died(pid);
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
