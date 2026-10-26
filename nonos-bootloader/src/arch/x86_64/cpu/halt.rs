// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Stopping the CPU.

/// Interrupts off and halted, for good.
#[inline(always)]
pub fn halt_forever() -> ! {
    loop {
        // SAFETY: the loader is done; nothing runs after this.
        unsafe {
            core::arch::asm!("cli; hlt", options(nomem, nostack));
        }
    }
}

/// Reset by triple fault: an empty IDT, then a breakpoint nothing can take.
#[inline(always)]
pub fn reset() -> ! {
    // SAFETY: the machine is reset; nothing of this boot survives it.
    unsafe {
        core::arch::asm!(
            "lidt [rax]",
            in("rax") 0u64,
            options(nomem, nostack)
        );
        core::arch::asm!("int3", options(nomem, nostack));
    }
    loop {
        core::hint::spin_loop();
    }
}
