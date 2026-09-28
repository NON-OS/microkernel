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

//! Each thread's alternate signal stack, as `sigaltstack` sets it: where a
//! handler installed with SA_ONSTACK is entered, so that a program whose own
//! stacks are small or are not its to use (a Go goroutine's) never has a
//! handler run on them. Linux keeps one per thread: a fork gives the child
//! the forking thread's, a new thread starts with none, and execve clears it.

use super::sigqueue::Signals;

/// `sigaltstack`'s flags, from Linux's `include/uapi/linux/signal.h`.
pub const SS_ONSTACK: u32 = 1;
pub const SS_DISABLE: u32 = 2;

/// Where a thread's alternate stack is: its lowest address and its size.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct AltStack {
    pub sp: u64,
    pub size: u64,
}

impl AltStack {
    /// True when `rsp` is on this stack, as Linux's `on_sig_stack` tests it:
    /// above the base, and no further than its size.
    pub fn holds(&self, rsp: u64) -> bool {
        rsp > self.sp && rsp - self.sp <= self.size
    }
}

impl Signals {
    pub fn stack(&self, tid: u32) -> Option<AltStack> {
        self.stacks.iter().find(|(t, _)| *t == tid).map(|(_, s)| *s)
    }

    /// Set or, with None, disable a thread's alternate stack.
    pub fn set_stack(&mut self, tid: u32, stack: Option<AltStack>) {
        self.stacks.retain(|(t, _)| *t != tid);
        if let Some(s) = stack {
            self.stacks.push((tid, s));
        }
    }

    /// A forked child's copy: its one thread has the forking thread's stack.
    pub fn stack_for_child(&mut self, forker: u32, child: u32) {
        let kept = self.stack(forker);
        self.stacks.clear();
        self.set_stack(child, kept);
    }

    /// execve: the program that follows has set no stack.
    pub fn clear_stacks(&mut self) {
        self.stacks.clear();
    }
}
