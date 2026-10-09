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

//! Writing IA32_PAT on every CPU, the same value everywhere as SDM Vol. 3A
//! 12.12.4 requires: a page seen through two different tables has no
//! defined type.

use super::value::with_wc;
use crate::arch::x86_64::cpu::{cpuid, rdmsr, wrmsr};
use core::sync::atomic::{AtomicBool, Ordering};

const IA32_PAT: u32 = 0x277;
/// CPUID.01H:EDX bit 16, the PAT feature flag.
const CPUID_PAT: u32 = 1 << 16;

static WC_READY: AtomicBool = AtomicBool::new(false);

/// Boot CPU, before the framebuffer is mapped and before any AP starts.
/// Returns whether a page can now be mapped write-combining.
///
/// # Safety
/// Runs on the boot CPU while no other CPU is running kernel code.
pub unsafe fn program_boot() -> bool {
    if cpuid(1).3 & CPUID_PAT == 0 {
        return false;
    }
    // SAFETY: the CPU has the PAT (checked above) and nothing maps a page
    // with PWT alone yet, so no live mapping changes type.
    unsafe { write() };
    WC_READY.store(true, Ordering::Release);
    true
}

/// On each AP during its bring-up: the table the boot CPU wrote, if any.
///
/// # Safety
/// Runs once per AP with interrupts off, before it runs any thread.
pub unsafe fn mirror_on_ap() {
    if WC_READY.load(Ordering::Acquire) {
        // SAFETY: every CPU has the PAT when the boot CPU has it.
        unsafe { write() };
    }
}

/// Whether entry 1 is write-combining on every CPU that runs threads.
pub fn wc_ready() -> bool {
    WC_READY.load(Ordering::Acquire)
}

/// # Safety
/// The CPU must have the PAT.
unsafe fn write() {
    // Lines cached under the old table are written back before and after
    // the change, as Linux does around its PAT write in cache_cpu_init.
    // SAFETY: WBINVD and WRMSR at ring 0 touch only this CPU's caches and
    // its PAT; the value keeps entries 0, 2 and 3 as reset left them.
    unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
    wrmsr(IA32_PAT, with_wc(rdmsr(IA32_PAT)));
    unsafe { core::arch::asm!("wbinvd", options(nostack, preserves_flags)) };
}
