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

//! TLB invalidation. x86_64 drops a single translation with `invlpg` and the
//! whole set, GLOBAL entries included, by toggling CR4.PGE and rewriting CR3;
//! aarch64 issues the inner-shareable `tlbi` variants, which the arch layer
//! already wraps with the `dsb`/`isb` barriers the architecture requires
//! around them.

/// Drop the translation for one virtual page.
#[inline]
pub fn invalidate_page(va: u64) {
    #[cfg(target_arch = "x86_64")]
    {
        // SAFETY: invlpg only invalidates a translation; it never dereferences
        // the address, so an unmapped `va` is harmless.
        unsafe {
            core::arch::asm!("invlpg [{}]", in(reg) va, options(nostack, preserves_flags));
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::mmu::flush_tlb_page(va);
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = va;
    }
}

/// Drop every translation, GLOBAL ones included.
#[inline]
pub fn invalidate_all() {
    #[cfg(target_arch = "x86_64")]
    {
        // Kernel PTEs are GLOBAL and a CR3 write keeps those while CR4.PGE is
        // set, so PGE is cleared and put back, which drops every entry; with
        // PGE off the CR3 write does. Masked so the task cannot move CPUs and
        // write this CR4 on another; an NMI here leaves CR4 as it found it.
        // SAFETY: CR4 changes only in PGE and returns to its value, CR3 is the
        // live table unchanged, and the flags are restored.
        unsafe {
            core::arch::asm!(
                "pushfq",
                "cli",
                "mov {cr4}, cr4", "mov {tmp}, {cr4}", "and {tmp}, {not_pge}",
                "mov cr4, {tmp}", "mov cr4, {cr4}",
                "mov {tmp}, cr3", "mov cr3, {tmp}",
                "popfq",
                cr4 = out(reg) _,
                tmp = out(reg) _,
                not_pge = in(reg) !(1u64 << 7),
            );
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        crate::arch::aarch64::mmu::flush_tlb_all();
    }
}
