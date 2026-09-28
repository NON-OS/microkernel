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

//! `vfork`, and the start `clone` shares with it when it makes a process
//! rather than a thread (vfork_clone). The child is a copy, as fork's is:
//! vfork's child shares its parent's memory on Linux, but it may only exec or
//! exit, and the parent sleeps until it does, so what either sees is the
//! same. That is what Go's os/exec asks for with clone(CLONE_VFORK|CLONE_VM),
//! and musl's posix_spawn with a stack of the child's own, which the kernel's
//! fork starts it on. The parent parks here and the family answers it with
//! the child's pid when the child's exec succeeds or the child ends.

use crate::linux::abi::errno;
use crate::linux::guest::sigstate::SIGCHLD;
use crate::linux::guest::Guest;
use crate::linux::serve::Answer;

use super::fork_child::fork_child;

pub fn vfork(guest: &mut Guest, caller: u32) -> Answer {
    start(guest, caller, 0, SIGCHLD, true, |_| {})
}

/// Fork a child and answer with its pid, or park the caller until the child
/// execs or ends when `parks`.
pub(super) fn start(
    guest: &mut Guest,
    caller: u32,
    stack: u64,
    signal: u8,
    parks: bool,
    prep: impl FnOnce(&mut Guest),
) -> Answer {
    let child = match fork_child(guest, caller, stack, signal, prep) {
        Ok(c) => c,
        Err(e) => return Answer::value(e),
    };
    if !parks {
        return Answer::value(errno::ok(u64::from(child)));
    }
    if let Some(c) = guest.forked.last_mut() {
        c.signals.vfork = Some(caller);
    }
    Answer::Park
}
