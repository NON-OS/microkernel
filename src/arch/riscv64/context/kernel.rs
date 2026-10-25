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

//! The kernel-side context the scheduler saves before it switches away: the
//! setjmp half of a switch, not the trap frame. A thread saves where it
//! stands, the scheduler runs something else, and a later restore lands back
//! after the save with the stack it left. Only what the RISC-V calling
//! convention keeps across a call is here, s0 to s11, ra and sp: the caller
//! of `save_to` expects nothing else to survive.

/// A saved kernel execution point. Restoring it returns from `save_to` a
/// second time, which is how the scheduler tells a fresh save from a resume.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Context {
    /// s0 to s11; s0 is the frame pointer.
    pub s: [u64; 12],
    /// The return address, where `restore_from` lands.
    pub ra: u64,
    pub sp: u64,
}

impl Context {
    /// Save the calling frame into `ctx`.
    ///
    /// # Safety
    /// `ctx` must point to a writable, aligned `Context`.
    #[unsafe(naked)]
    #[rustfmt::skip]
    pub unsafe extern "C" fn save_to(ctx: *mut Context) {
        // a0 holds `ctx`; offsets follow the fields, eight bytes each.
        core::arch::naked_asm!(
            "sd s0, 0(a0)", "sd s1, 8(a0)", "sd s2, 16(a0)", "sd s3, 24(a0)",
            "sd s4, 32(a0)", "sd s5, 40(a0)", "sd s6, 48(a0)", "sd s7, 56(a0)",
            "sd s8, 64(a0)", "sd s9, 72(a0)", "sd s10, 80(a0)", "sd s11, 88(a0)",
            "sd ra, 96(a0)", "sd sp, 104(a0)",
            "ret",
        );
    }

    /// Resume a saved point. It returns to whoever called `save_to`.
    ///
    /// # Safety
    /// `ctx` was saved by `save_to`, and its stack is still mapped and still
    /// this thread's.
    #[unsafe(naked)]
    #[rustfmt::skip]
    pub unsafe extern "C" fn restore_from(ctx: *const Context) -> ! {
        core::arch::naked_asm!(
            "ld s0, 0(a0)", "ld s1, 8(a0)", "ld s2, 16(a0)", "ld s3, 24(a0)",
            "ld s4, 32(a0)", "ld s5, 40(a0)", "ld s6, 48(a0)", "ld s7, 56(a0)",
            "ld s8, 64(a0)", "ld s9, 72(a0)", "ld s10, 80(a0)", "ld s11, 88(a0)",
            "ld ra, 96(a0)", "ld sp, 104(a0)",
            "ret",
        );
    }
}
