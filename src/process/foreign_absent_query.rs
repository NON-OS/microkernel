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

//! The rest of the foreign surface where nothing is ever hosted: no pid is a
//! guest, none has a supervisor, and the calls that act on one refuse.

use crate::syscall::microkernel::errnos::ERRNO_NOSYS;

pub fn is_foreign(_pid: u32) -> bool {
    false
}

pub fn supervisor_of(_pid: u32) -> Option<u32> {
    None
}

/// No peer call can be in flight, so there is nothing to hold off.
pub fn without_peer_calls<R>(f: impl FnOnce() -> R) -> R {
    f()
}

pub fn note_signal_death(_pid: u32, _code: i32) {}

pub fn fill_guest(_caller: u32, _pid: u64, _guest_addr: u64, _bytes: &[u8]) -> Result<(), i64> {
    Err(ERRNO_NOSYS)
}

pub fn sys_foreign_context(_pid: u64, _out: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_signal(_pid: u64, _regs: u64, _kind: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_interrupt(_pid: u64) -> i64 {
    ERRNO_NOSYS
}
