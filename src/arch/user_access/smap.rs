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

//! SMAP: `AC` in RFLAGS decides whether a privileged access to a user page
//! faults. STAC and CLAC are NOPs while CR4.SMAP is clear, but #UD on a CPU
//! whose CPUID does not list SMAP at all (pre-Broadwell parts, `-cpu
//! max,-smap`). Such a CPU has no SMAP to open, so the window there is a
//! no-op rather than an invalid-opcode fault on the first user copy.

use core::sync::atomic::{AtomicU8, Ordering};

use crate::arch::x86_64::time::tsc::cpuid;

const UNKNOWN: u8 = 0;
const PRESENT: u8 = 1;
const ABSENT: u8 = 2;
/// CPUID is a VM exit under virtualization, and this sits on every user copy.
static SMAP: AtomicU8 = AtomicU8::new(UNKNOWN);

#[inline(always)]
pub(super) fn allow() {
    if present() {
        // SAFETY: CPUID lists SMAP, so STAC is defined. It sets one RFLAGS
        // bit; `deny` follows by the guard that called this.
        unsafe { core::arch::asm!("stac", options(nomem, nostack, preserves_flags)) };
    }
}

#[inline(always)]
pub(super) fn deny() {
    if present() {
        // SAFETY: as for `allow`. Clearing an already clear AC is harmless,
        // which is what makes the guard idempotent.
        unsafe { core::arch::asm!("clac", options(nomem, nostack, preserves_flags)) };
    }
}

#[inline(always)]
fn present() -> bool {
    match SMAP.load(Ordering::Relaxed) {
        PRESENT => true,
        ABSENT => false,
        _ => probe(),
    }
}

/// Leaf 7 subleaf 0, EBX bit 20. Every CPU reads the same answer, so a race
/// between two first callers stores the same value twice.
#[cold]
fn probe() -> bool {
    let present = cpuid(0, 0).0 >= 7 && cpuid(7, 0).1 & (1 << 20) != 0;
    SMAP.store(if present { PRESENT } else { ABSENT }, Ordering::Relaxed);
    present
}
