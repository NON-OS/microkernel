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

use crate::kernel_core::surface_registry::attach_map;
use crate::kernel_core::surface_registry::pin::drop_attach::drop_attach;
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;

use super::super::surface_ops::{map_err, EPERM, ESRCH};

pub(in super::super) fn do_release(handle: u64) -> SyscallResult {
    // Surfaces are held by the process, not the thread that asked (L36).
    let Some(pid) = current_pid().map(crate::hardware::broker::holder_of) else {
        return errno(ESRCH);
    };
    // Only a holder (the owner or an attacher, both of which have an attach
    // record for this handle) may drop a reference. Without this, any capsule
    // could guess a handle -- epochs are small and there are only 256 slots --
    // and force-free another capsule's surface, desyncing its refcount and
    // invalidating its sid out from under it.
    if attach_map::lookup(pid, handle).is_none() {
        return errno(EPERM);
    }
    // The owner letting go takes back what it named.
    crate::kernel_core::surface_registry::share::owner_releases(pid, handle);
    match drop_attach(pid, handle) {
        Ok(n) => SyscallResult::success_audited(n as i64),
        Err(e) => errno(map_err(e)),
    }
}
