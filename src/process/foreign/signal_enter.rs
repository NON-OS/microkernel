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

//! A parked guest leaving into a signal handler, or back out of one, with
//! its FPU state kept by `signal_fpu`.

use crate::arch::context::SavedUser;
use crate::process::userspace::restore_user_context_iretq;

use super::signal_fpu::{enter_fpu, leave_fpu};

pub(super) fn deliver(pid: u32, ctx: SavedUser) -> ! {
    enter_fpu(pid);
    resume(ctx)
}

pub(super) fn sigreturn(pid: u32, ctx: SavedUser) -> ! {
    leave_fpu(pid);
    resume(ctx)
}

fn resume(ctx: SavedUser) -> ! {
    crate::arch::context::set_user_tls(ctx.fs_base);
    /*
     * SAFETY: eK@nonos.systems - `ctx` came through `from_words`: user
     * selectors, rip and rsp in the low half, flags masked to the program's
     * own. The address space is this pid's, already on cr3, as in exec_enter.
     */
    unsafe { restore_user_context_iretq(&ctx) }
}
