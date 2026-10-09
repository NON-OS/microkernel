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

//! What an interrupted call answers, as Linux decides it (guest::sigrestart):
//! a transfer that moved something answers that count; reads and writes,
//! the socket calls, the lock waits, wait4, waitid and an untimed futex wait
//! run again under SA_RESTART unless they had a limit of their own; every
//! other wait, and every one without SA_RESTART, answers EINTR. A relative
//! nanosleep writes the time it had left first (deliver_rem).

use nonos_libc::ForeignRegs;

use crate::linux::guest::sigrestart::{after, After};

pub const R10: usize = 2;
pub const RSI: usize = 9;
pub const RAX: usize = 13;
const RIP: usize = 16;
/// The length of `syscall`, which a restart steps back over.
const SYSCALL_LEN: u64 = 2;

/// Set rax to what the handler returns into: the call again, or its answer.
/// The parked frame's rax is still the call's number.
pub fn rewind(regs: &mut ForeignRegs, flags: u64, moved: u64, timed: bool) {
    match after(regs[RAX], flags, regs[R10], moved, timed) {
        After::Again => regs[RIP] = regs[RIP].wrapping_sub(SYSCALL_LEN),
        After::Answer(value) => regs[RAX] = value,
    }
}
