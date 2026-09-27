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

//! A parked guest leaving into a signal handler, or back out of one. The FPU
//! state stays in the kernel: entering saves the thread's own and gives the
//! handler a clean unit, returning puts it back, and no guest can forge it.

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

use crate::arch::context::SavedUser;
use crate::process::signal::SIGSEGV;
use crate::process::userspace::{restore_user_context_iretq, types::FpuState};

/// Handlers nested deeper than this end the thread.
const DEPTH: usize = 8;

static SAVED: Mutex<BTreeMap<u32, Vec<FpuState>>> = Mutex::new(BTreeMap::new());

pub(super) fn deliver(pid: u32, ctx: SavedUser) -> ! {
    let mut fpu = FpuState::default();
    fpu.save();
    let pushed = {
        let mut saved = SAVED.lock();
        let stack = saved.entry(pid).or_default();
        let room = stack.len() < DEPTH;
        if room {
            stack.push(fpu);
        }
        room
    };
    if !pushed {
        crate::process::terminate_current_with_signal(SIGSEGV);
    }
    FpuState::init();
    resume(ctx)
}

pub(super) fn sigreturn(pid: u32, ctx: SavedUser) -> ! {
    let top = SAVED.lock().get_mut(&pid).and_then(|s| s.pop());
    match top {
        Some(fpu) => fpu.restore(),
        // A return with nothing delivered: not a frame this kernel made.
        None => crate::process::terminate_current_with_signal(SIGSEGV),
    }
    resume(ctx)
}

/// Exec and exit leave no handler to return from.
pub(super) fn forget(pid: u32) {
    SAVED.lock().remove(&pid);
}

fn resume(ctx: SavedUser) -> ! {
    crate::arch::context::set_user_tls(ctx.fs_base);
    // SAFETY: eK@nonos.systems - `ctx` came through `from_words`: user
    // selectors, rip and rsp in the low half, flags masked to the program's
    // own. The address space is this pid's, already on cr3, as in exec_enter.
    unsafe { restore_user_context_iretq(&ctx) }
}
