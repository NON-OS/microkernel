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

use crate::kernel_core::surface_registry::{
    attach_surface, lookup_owned, share_surface, SurfaceDescriptor,
};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::write_user_value;

use super::super::surface_ops::{map_err, EFAULT, ESRCH};

pub(in super::super) fn do_share(sid: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    let handle = match lookup_owned(pid, sid) {
        Ok(h) => h,
        Err(e) => return errno(map_err(e)),
    };
    match share_surface(pid, handle) {
        Ok(h) => SyscallResult::success_audited(h as i64),
        Err(e) => errno(map_err(e)),
    }
}

pub(in super::super) fn do_attach(handle: u64, out_desc_ptr: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    let mut desc = SurfaceDescriptor::default();
    match attach_surface(pid, handle, &mut desc) {
        Ok(va) => {
            if out_desc_ptr != 0 && write_user_value(out_desc_ptr, &desc).is_err() {
                return errno(EFAULT);
            }
            SyscallResult::success_audited(va as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}
