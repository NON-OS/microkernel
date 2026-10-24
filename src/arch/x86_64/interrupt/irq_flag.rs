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

//! The one place RFLAGS.IF is set and cleared.
//!
//! Every `cli` and `sti` in the tree used to be its own `asm!` marked `nomem`,
//! which tells the compiler the instruction touches no memory, so it was free
//! to move loads and stores across it. A critical section bracketed that way
//! could have its accesses scheduled before the `cli` or after the `sti`,
//! where an interrupt handler sees them half done. Without `nomem` each one
//! is a compiler barrier: no memory access crosses it in either direction.
//!
//! The fused sequences that rely on STI's one-instruction shadow
//! (`sti; hlt`) cannot be split into these calls and keep their own asm,
//! also without `nomem`.

/// Mask maskable interrupts on this CPU.
#[inline(always)]
pub fn cli() {
    // SAFETY: CLI clears IF and nothing else; ring 0 only, as all callers
    // are. IF is not one of the status flags `preserves_flags` covers.
    unsafe { core::arch::asm!("cli", options(nostack, preserves_flags)) };
}

/// Unmask maskable interrupts on this CPU. A pending interrupt is taken
/// after the next instruction, not this one.
#[inline(always)]
pub fn sti() {
    // SAFETY: STI sets IF and nothing else; every gate the IDT holds is
    // installed before any caller reaches this.
    unsafe { core::arch::asm!("sti", options(nostack, preserves_flags)) };
}
