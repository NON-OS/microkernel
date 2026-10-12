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

//! `MkThreadGroup(pid) -> thread group`: the process a sender belongs to.
//! MkIpcRecvFrom reports the sending thread, and a service that answers
//! or asks from a worker thread is still that service; the registry and
//! the claim holder compare by thread group (`peers_check.rs`,
//! `broker/acpi/same_group.rs`), and a server checking its own senders
//! needs the same answer. A thread whose group reads 0 is its own group.
//! ENOENT for a pid that names no process.

use crate::syscall::microkernel::errnos::ERRNO_NOENT;

pub fn sys_thread_group(pid: u64) -> i64 {
    let Ok(pid) = u32::try_from(pid) else { return ERRNO_NOENT };
    match crate::process::with_process(pid, |pcb| pcb.thread_group_id()) {
        Some(0) => i64::from(pid),
        Some(g) => i64::from(g),
        None => ERRNO_NOENT,
    }
}
