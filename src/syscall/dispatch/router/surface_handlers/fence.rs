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

//! MkSurfaceFenceRelease and MkSurfaceFenceWait: a surface's release fence
//! (kernel_core::surface_registry::fence).

use crate::kernel_core::surface_registry::{release_fence, wait_fence};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::microkernel::narrow::u32_arg;
use crate::syscall::SyscallResult;

use super::super::surface_ops::{map_err, EINVAL, ESRCH};

/// The caller's process, holding `handle` attached, is done reading it.
pub(in super::super) fn do_fence_release(handle: u64) -> SyscallResult {
    // Surfaces are held by the process, not the thread that asked.
    let Some(holder) = current_pid().map(crate::hardware::broker::holder_of) else {
        return errno(ESRCH);
    };
    match release_fence(holder, handle) {
        Ok(n) => SyscallResult::success_audited(i64::from(n)),
        Err(e) => errno(map_err(e)),
    }
}

/// The caller's process, owning `sid`, waits for its count to pass `seen`.
pub(in super::super) fn do_fence_wait(sid: u64, seen: u64, timeout_ms: u64) -> SyscallResult {
    let Some(pid) = current_pid() else {
        return errno(ESRCH);
    };
    let Some(seen) = u32_arg(seen) else {
        return errno(EINVAL);
    };
    let holder = crate::hardware::broker::holder_of(pid);
    match wait_fence(holder, pid, sid, seen, timeout_ms) {
        Ok(n) => SyscallResult::success_audited(i64::from(n)),
        Err(e) => errno(map_err(e)),
    }
}
