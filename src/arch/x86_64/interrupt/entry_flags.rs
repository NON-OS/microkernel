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

//! RFLAGS.AC on entry to an `x86-interrupt` handler.
//!
//! A gate leaves AC as the interrupted code had it, and ring 3 may set it
//! with popf, which turns SMAP off for the whole handler. The assembly
//! trampolines clear it with `CLEAR_AC` (`asm/clear_ac.inc`); the handlers
//! the IDT installs as Rust functions call this first instead.

/// Clear AC through the stack. CLAC would be shorter but is #UD on a CPU
/// without SMAP. iretq puts the interrupted RFLAGS back, AC included.
#[inline(always)]
pub fn clear_ac() {
    // SAFETY: pushes and pops one word of the current kernel stack and
    // changes no flag but AC; IF is written back as it was read.
    unsafe {
        core::arch::asm!(
            "pushfq",
            "and qword ptr [rsp], {mask}",
            "popfq",
            mask = const !(1i32 << 18),
        );
    }
}
