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

//! The `rt_sigframe` x86-64 puts on a thread's stack to enter a signal handler,
//! and where to read it back on return. Pure, so the layout is checked without
//! a guest. It matches Linux `struct rt_sigframe`: pretcode u64, ucontext at
//! +8, siginfo at +312. Within the ucontext, `uc_stack` is at +16, the
//! sigcontext (`uc_mcontext`) at +40 and `uc_sigmask` at +296, as musl, glibc
//! and Go all read them; the sigcontext starts with the 18 words
//! `mk_foreign_context` uses, in that order. A handler that reads or edits its
//! context (Go's does, to preempt) finds each register where Linux puts it.

mod build;
mod layout;
mod place;

pub use build::build;
pub use layout::{returned, SIGCONTEXT_OFF, WORDS};
