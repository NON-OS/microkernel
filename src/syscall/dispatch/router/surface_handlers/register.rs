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

use crate::kernel_core::surface_registry::pin::register_check::register_owned;
use crate::kernel_core::surface_registry::SurfaceDescriptor;
use crate::process::current_pid;
use crate::syscall::dispatch::util::errno;
use crate::syscall::SyscallResult;
use crate::usercopy::read_user_value;
use core::sync::atomic::AtomicBool;

use super::super::surface_ops::{map_err, EFAULT, ESRCH};
use super::display_mode::log_display_mode;
use super::frames::user_frames;

static FIRST_SURFACE_REGISTER: AtomicBool = AtomicBool::new(false);

pub(in super::super) fn do_register(desc_ptr: u64) -> SyscallResult {
    // Surfaces are held by the process, not the thread that asked (L36).
    let Some(pid) = current_pid().map(crate::hardware::broker::holder_of) else {
        return errno(ESRCH);
    };
    let desc: SurfaceDescriptor = match read_user_value(desc_ptr) {
        Ok(v) => v,
        Err(_) => return errno(EFAULT),
    };
    let frames = match user_frames(&desc) {
        Ok(frames) => frames,
        Err(refused) => return refused,
    };
    // Only frames that are the caller's own to give, the owner's attach
    // record made with the slot (L18a).
    match register_owned(pid, &desc, frames) {
        Ok((sid, _)) => {
            crate::sys::bench::mark_once(&FIRST_SURFACE_REGISTER, b"surface_register_first");
            log_display_mode(&desc);
            SyscallResult::success_audited(sid as i64)
        }
        Err(e) => errno(map_err(e)),
    }
}
