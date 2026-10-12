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

//! MkSurfaceShare(sid, receiver_pid).

use crate::kernel_core::surface_registry::{lookup_owned, share_surface, share_surface_to};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::microkernel::narrow::u32_arg;
use crate::syscall::SyscallResult;

use super::surface_ops::{map_err, EINVAL, ESRCH};

/*
 * The second argument always stood in the ABI as the target pid, and was
 * ignored: any process could attach a handle it guessed. Now it names the
 * one process that may attach (L18c). 0 is what every older caller sends,
 * and they all shared with the compositor, so 0 means the compositor.
 */
pub(super) fn do_share(sid: u64, receiver: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    let Some(receiver) = u32_arg(receiver) else {
        return errno(EINVAL);
    };
    let handle = match lookup_owned(pid, sid) {
        Ok(h) => h,
        Err(e) => return errno(map_err(e)),
    };
    let shared = match receiver {
        0 => share_surface(pid, handle),
        r => share_surface_to(pid, handle, r),
    };
    match shared {
        Ok(h) => SyscallResult::success_audited(h as i64),
        Err(e) => errno(map_err(e)),
    }
}
