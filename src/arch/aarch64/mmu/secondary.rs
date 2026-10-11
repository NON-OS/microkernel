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

//! What a secondary CPU loads to turn its MMU on exactly as the boot CPU did.
//!
//! A secondary starts with translation and its data cache off. Until both are
//! on, its loads and stores go straight to memory past the boot CPU's caches,
//! so neither sees what the other wrote, and its atomics act on memory with
//! Device attributes, where exclusive accesses are not guaranteed to work
//! (Arm ARM, DDI 0487, the synchronization primitives and memory types). So
//! the entry stub loads this block and switches the MMU on before any Rust
//! runs, and the boot CPU writes the block out to memory first: read with the
//! cache off, a value still in the boot CPU's cache is not seen.

use core::sync::atomic::{AtomicU64, Ordering};

/// SCTLR_EL1 M, C and I: translation, data and instruction caching.
const SCTLR_MMU_ON: u64 = 1 << 0 | 1 << 2 | 1 << 12;

/// MAIR_EL1, TCR_EL1, TTBR0_EL1, TTBR1_EL1 and the SCTLR_EL1 bits to set, in
/// the order `_aarch64_secondary_start` loads them.
#[no_mangle]
static AARCH64_MMU_CONTEXT: [AtomicU64; 5] = [const { AtomicU64::new(0) }; 5];

/// Record the boot CPU's translation registers for the secondaries, and write
/// the record to the point of coherency. Call before any CPU_ON.
pub fn publish_for_secondaries() {
    let (mair, tcr, ttbr0, ttbr1): (u64, u64, u64, u64);
    // SAFETY: reading the calling CPU's own translation registers at EL1.
    unsafe {
        core::arch::asm!(
            "mrs {0}, mair_el1",
            "mrs {1}, tcr_el1",
            "mrs {2}, ttbr0_el1",
            "mrs {3}, ttbr1_el1",
            out(reg) mair,
            out(reg) tcr,
            out(reg) ttbr0,
            out(reg) ttbr1,
            options(nomem, nostack, preserves_flags),
        );
    }
    for (slot, value) in AARCH64_MMU_CONTEXT.iter().zip([mair, tcr, ttbr0, ttbr1, SCTLR_MMU_ON]) {
        slot.store(value, Ordering::Release);
    }
    let at = AARCH64_MMU_CONTEXT.as_ptr() as u64;
    crate::arch::cache_flush::flush_and_fence(
        at,
        core::mem::size_of_val(&AARCH64_MMU_CONTEXT) as u64,
    );
}
