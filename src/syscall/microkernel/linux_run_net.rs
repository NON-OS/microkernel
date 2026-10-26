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

//! `MkLinuxRunNet`: the Linux run role asks to be started again with Network.
//!
//! init cannot read a policy's mask, so the run role starts without Network.
//! When the program it is about to run has NET_ANON, the role names its own
//! pid here and exits, and init starts the same run as app.linux.run.net
//! (src/userspace/capsule_linux/run_net.rs). Only the process registered as
//! app.linux.run may ask, only for itself, and only for the package init
//! started it for, so a capsule that holds ForeignExec gains nothing by
//! asking, and the network role re-binds the policy and exits without
//! NET_ANON. The process registered as app.linux.term may ask too, for
//! itself: its Terminal is then given one start of the same argv in the
//! terminal's network role (capsule_linux/term_ticket.rs).

use crate::syscall::microkernel::errnos::{ERRNO_BUSY, ERRNO_PERM};
use crate::userspace::capsule_linux::{asked_by_terminal_role, run_net_busy, run_package_of};

/// `MkLinuxRunNet(pid)`. 0 once the run is queued for init, or a terminal
/// run's ticket written; EPERM when `pid` is not the caller, or the caller
/// is neither the run role started by init for a package nor the terminal
/// role; EBUSY when a run already holds the network role, a ticket waits,
/// or init's queue is full or already holds this run.
pub fn sys_linux_run_net(pid: u64) -> i64 {
    let Some(caller) = caller_group() else { return ERRNO_PERM };
    if pid != u64::from(caller) {
        return ERRNO_PERM;
    }
    // The Terminal's own `linux` run: a ticket for its Terminal (term_ticket.rs).
    if let Some(rc) = asked_by_terminal_role(caller) {
        return rc;
    }
    let Some(package) = run_package_of(caller) else { return ERRNO_PERM };
    if run_net_busy() {
        return ERRNO_BUSY;
    }
    match crate::userspace::init::request_net_run(package) {
        true => 0,
        false => ERRNO_BUSY,
    }
}

/// The caller's process, not the thread that asked.
fn caller_group() -> Option<u32> {
    let pid = crate::process::current_pid()?;
    crate::process::with_process(pid, |pcb| pcb.thread_group_id()).filter(|&g| g != 0).or(Some(pid))
}
