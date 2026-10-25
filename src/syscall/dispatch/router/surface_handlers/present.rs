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

use crate::kernel_core::surface_registry::{lookup_attached_va, wait_for_vsync};
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::microkernel::narrow::u32_arg;
use crate::syscall::SyscallResult;
use core::sync::atomic::AtomicBool;

use super::super::surface_ops::{map_err, EINVAL, ESRCH};

static FIRST_SURFACE_PRESENT: AtomicBool = AtomicBool::new(false);

pub(in super::super) fn do_present(handle: u64, x: u64, y: u64, w: u64, h: u64) -> SyscallResult {
    // Surfaces are held by the process, not the thread that asked (L36).
    let Some(pid) = current_pid().map(crate::hardware::broker::holder_of) else {
        return errno(ESRCH);
    };
    let (base_va, byte_len) = match lookup_attached_va(pid, handle) {
        Some(v) => v,
        None => return errno(EINVAL),
    };
    let result = super::super::graphics_present::handle(0, base_va, byte_len as usize, x, y, w, h);
    if !result.is_error() {
        crate::sys::bench::mark_once(&FIRST_SURFACE_PRESENT, b"surface_present_first");
    }
    result
}

pub(in super::super) fn do_vsync_wait(display: u64) -> SyscallResult {
    let pid = match current_pid() {
        Some(p) => p,
        None => return errno(ESRCH),
    };
    let Some(display) = u32_arg(display) else {
        return errno(EINVAL);
    };
    match wait_for_vsync(display, pid) {
        Ok(deadline) => SyscallResult::success_audited(deadline as i64),
        Err(e) => errno(map_err(e)),
    }
}
