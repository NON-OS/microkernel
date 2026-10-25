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

//! CR4.PGE, set by the kernel on every CPU rather than left as firmware and
//! the trampoline had it, so the boot CPU and every AP cache kernel entries
//! the same way.
//!
//! Kernel PTEs carry GLOBAL and no user PTE does, so with PGE on a CR3 write
//! keeps the kernel's entries and drops only the address space being left.
//! That is also why a full flush cannot be a CR3 write once PGE is on: it
//! would keep exactly the kernel entries a kernel shootdown exists to drop.
//! `arch::paging::invalidate_all` toggles PGE for that reason.

use core::arch::asm;

use super::super::super::super::constants::CR4_PGE;

/// Turn PGE on where CPUID offers it. Returns the bit as read back.
pub(in super::super) fn enable(offered: bool) -> bool {
    let cr4 = super::read();
    if offered && cr4 & CR4_PGE == 0 {
        // SAFETY: ek@nonos.systems - CPUID leaf 1 reported PGE, so the bit is
        // not reserved on this part. Setting it flushes nothing and changes no
        // translation; it only lets GLOBAL entries outlive a CR3 write, and
        // every full flush in the kernel already drops them by toggling it.
        unsafe {
            asm!("mov cr4, {}", in(reg) cr4 | CR4_PGE, options(nomem, nostack, preserves_flags))
        };
    }
    super::read() & CR4_PGE != 0
}

#[cfg(all(
    feature = "nonos-smp-pge-proof",
    any(feature = "nonos-production", feature = "nonos-release")
))]
compile_error!("nonos-smp-pge-proof is a boot proof and never part of a production build.");

/// `nonos-smp-pge-proof` only: the first CPU here, the boot CPU, gets PGE set
/// before `enable` looks, as firmware may leave it, while every AP comes from
/// a trampoline that leaves it clear. A boot then shows whether every CPU
/// still ends with the same `pge=`.
#[cfg(feature = "nonos-smp-pge-proof")]
pub(in super::super) fn proof_boot_cpu_only() {
    use core::sync::atomic::{AtomicBool, Ordering};
    static DONE: AtomicBool = AtomicBool::new(false);
    if DONE.swap(true, Ordering::AcqRel) {
        return;
    }
    let was = super::read() & CR4_PGE != 0;
    let now = enable(core::arch::x86_64::__cpuid(1).edx & (1 << 13) != 0);
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[PGE-PROOF] boot cpu pge forced before handoff, was=").dec(was as u64);
    l.str(b" now=").dec(now as u64);
    l.end();
}
