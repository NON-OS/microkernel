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

//! The receiver a share names: a live process other than the caller's,
//! never a guest. Naming the caller's own process records nobody: it only
//! hands the owner its handle, which it may attach and present anyway.

use crate::process::{get_process, with_process, ProcessState};
use crate::syscall::microkernel::narrow::u32_arg;

use super::super::surface_ops::{EINVAL, ESRCH};

/// Some(receiver) to record, or None for the caller's own process.
pub(super) fn receiver(caller: u32, target: u64) -> Result<Option<u32>, i32> {
    let target = u32_arg(target).filter(|&t| t != 0).ok_or(EINVAL)?;
    if !alive(target) {
        return Err(ESRCH);
    }
    if group(target) == group(caller) {
        return Ok(None);
    }
    if crate::process::foreign::is_foreign(target) {
        return Err(EINVAL);
    }
    Ok(Some(target))
}

fn alive(pid: u32) -> bool {
    get_process(pid).is_some_and(|p| {
        !matches!(*p.state.lock(), ProcessState::Zombie(_) | ProcessState::Terminated(_))
    })
}

fn group(pid: u32) -> u32 {
    with_process(pid, |p| p.thread_group_id()).filter(|&g| g != 0).unwrap_or(pid)
}
