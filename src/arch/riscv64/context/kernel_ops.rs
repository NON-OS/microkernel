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

//! The scheduler's side of a saved kernel context: making one to start a
//! thread at, taking and resuming one, and telling a resume from a save.

use core::sync::atomic::Ordering;

use super::kernel::Context;
use super::restored::{current_cpu_index, CONTEXT_JUST_RESTORED};

impl Context {
    /// A context that resumes at `entry` on the stack `stack_top`, with every
    /// callee-saved register clear.
    pub fn for_resume(stack_top: u64, entry: u64) -> Self {
        Self { s: [0; 12], ra: entry, sp: stack_top }
    }

    /// Save the calling frame and hand it back.
    #[inline(never)]
    pub fn save() -> Self {
        let mut ctx = Self::for_resume(0, 0);
        // SAFETY: `ctx` is a live, aligned local.
        unsafe { Self::save_to(&mut ctx as *mut Context) };
        ctx
    }

    /// Resume this context. Does not return. A null stack or entry is a
    /// corrupted save, not a thread, so the kernel stops rather than jump.
    pub fn restore(&self) -> ! {
        if self.sp == 0 || self.ra == 0 {
            crate::sys::serial::println(b"[FATAL] Context restore failed");
            crate::sys::serial::println(b"saved context has no stack or no entry");
            crate::arch::halt_loop()
        }
        super::restored::set_restored_flag();
        // SAFETY: the landing point was just checked, and the stack belongs to
        // the thread being resumed.
        unsafe { Self::restore_from(self as *const Context) }
    }

    /// True once per resume: it reads the flag and clears it.
    pub fn was_just_restored() -> bool {
        CONTEXT_JUST_RESTORED[current_cpu_index()].swap(false, Ordering::SeqCst)
    }

    pub fn clear_restored_flag() {
        CONTEXT_JUST_RESTORED[current_cpu_index()].store(false, Ordering::SeqCst);
    }
}
