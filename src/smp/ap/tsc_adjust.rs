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

//! Every CPU's IA32_TSC_ADJUST set to the boot CPU's, as Linux
//! tsc_store_and_check_tsc_adjust does.
//!
//! Uptime is the TSC of whichever CPU asks, less an anchor read on the boot
//! CPU. Firmware that leaves TSC_ADJUST different on an AP offsets its TSC,
//! so a process reads a different uptime on each CPU and a sleep or timeout
//! measured across a move ends early or very late. Only the APs are set: the
//! boot CPU's clock is already anchored.

use crate::arch::x86_64::boot::cpu_ops::{cpuid_count, rdmsr, wrmsr};
use crate::smp::MAX_CPUS;
use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Intel SDM vol. 4, table 2-2.
const IA32_TSC_ADJUST: u32 = 0x3B;
/// CPUID.(EAX=07H, ECX=0):EBX[1], SDM vol. 2A, CPUID.
const LEAF7_EBX_TSC_ADJUST: u32 = 1 << 1;

pub(super) static BOOT_VALUE: AtomicU64 = AtomicU64::new(0);
static BOOT_KNOWN: AtomicBool = AtomicBool::new(false);
/// What an AP's register held before it was set; reported once it can print.
pub(super) static WAS: [AtomicU64; MAX_CPUS] = [const { AtomicU64::new(0) }; MAX_CPUS];
pub(super) static SET: [AtomicBool; MAX_CPUS] = [const { AtomicBool::new(false) }; MAX_CPUS];

fn supported() -> bool {
    cpuid_count(0, 0).0 >= 7 && cpuid_count(7, 0).1 & LEAF7_EBX_TSC_ADJUST != 0
}

/// On the boot CPU, before any AP is started.
pub(crate) fn record_boot_cpu() {
    if supported() {
        // SAFETY: eK@nonos.systems - the MSR exists when CPUID says so, and
        // reading it changes nothing.
        BOOT_VALUE.store(unsafe { rdmsr(IA32_TSC_ADJUST) }, Ordering::Release);
        BOOT_KNOWN.store(true, Ordering::Release);
    }
}

/// On an AP, with interrupts masked, before it reads the time.
pub(super) fn align(cpu_id: u32) {
    let cpu = cpu_id as usize;
    if cpu >= MAX_CPUS || !BOOT_KNOWN.load(Ordering::Acquire) || !supported() {
        return;
    }
    let want = BOOT_VALUE.load(Ordering::Acquire);
    // SAFETY: eK@nonos.systems - the MSR exists when CPUID says so, and
    // reading it changes nothing.
    let was = unsafe { rdmsr(IA32_TSC_ADJUST) };
    if was != want {
        // SAFETY: eK@nonos.systems - writing it moves only this CPU's TSC, by
        // the difference, and nothing on this CPU has read the time yet.
        unsafe { wrmsr(IA32_TSC_ADJUST, want) };
        WAS[cpu].store(was, Ordering::Relaxed);
        SET[cpu].store(true, Ordering::Release);
    }
}
