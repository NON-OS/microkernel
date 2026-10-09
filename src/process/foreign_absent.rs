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

//! Supervised hosting where the kernel cannot supervise.
//!
//! A guest exists because of one path: a syscall number this kernel does not
//! know is parked and handed to the guest's supervisor instead of refused. That
//! path is x86_64 only. It reads the frame syscall.S pushes and the user rsp
//! from gs, and resumes an exec'd guest with iretq. The aarch64 SVC handler
//! answers every unknown number with ENOSYS and parks nothing, and the thread
//! pointer these calls set is never installed there (see arch/context/tls.rs).
//!
//! So every foreign and peer call answers ERRNO_NOSYS, as the port I/O calls do
//! on this architecture, rather than creating a guest nobody can supervise or
//! reporting success for a thread pointer that is dropped.

use crate::syscall::microkernel::errnos::ERRNO_NOSYS;

#[path = "foreign_absent_peer.rs"]
mod peer;
#[path = "foreign_absent_query.rs"]
mod query;
pub use peer::*;
pub use query::*;

pub fn sys_foreign_spawn(_name_ptr: u64, _name_len: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_start(_pid: u64, _entry: u64, _rsp: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_wait(_out_ptr: u64, _out_len: u64, _timeout_ms: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_reply(_pid: u64, _value: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_thread(_pid: u64, _entry: u64, _rsp: u64, _tls: u64, _tid: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_fork(_pid: u64, _rsp: u64) -> i64 {
    ERRNO_NOSYS
}

pub fn sys_foreign_exec(_pid: u64, _entry: u64, _rsp: u64) -> i64 {
    ERRNO_NOSYS
}

/// Drop `pid` from the foreign table. Spawn, thread and fork, the only paths
/// that record one, all refuse above, so none was ever recorded.
pub fn clear(pid: u32) {
    let _ = pid;
}
