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

//! What every CPU sets for itself. SPEC_CTRL and the predictors it governs
//! are per logical CPU: the boot CPU's write never reached the others, so
//! every AP ran capsules with STIBP and SSBD off.

use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use super::constants::{ARCH_CAP_IBRS_ALL, MSR_IA32_ARCH_CAPABILITIES, MSR_IA32_SPEC_CTRL};
use super::cpuid;
use super::ibrs::ibrs_enable;
use super::msr::{rdmsr, wrmsr};
use super::policy::{rewrite_on_entry, spec_ctrl_bits};

static BITS: AtomicU64 = AtomicU64::new(0);
static REWRITE_ON_ENTRY: AtomicBool = AtomicBool::new(false);

/// Decide once, on the boot CPU, before any AP is started. Returns the bits.
pub(super) fn decide() -> u64 {
    let ibrs = cpuid::has_ibrs_ibpb();
    let bits = spec_ctrl_bits(ibrs, cpuid::has_stibp(), cpuid::has_ssbd());
    REWRITE_ON_ENTRY.store(rewrite_on_entry(ibrs, enhanced_ibrs()), Ordering::Relaxed);
    BITS.store(bits, Ordering::Release);
    bits
}

/// Set this CPU's SPEC_CTRL bits. The boot CPU from `init`, every AP from
/// its bring-up, interrupts masked; bits the firmware set are kept.
pub fn apply_on_this_cpu() {
    let bits = BITS.load(Ordering::Acquire);
    if bits == 0 {
        return;
    }
    // SAFETY: a nonzero `bits` means CPUID reported the controls in it, so
    // SPEC_CTRL exists and each bit is defined. Only this CPU's copy changes.
    unsafe { wrmsr(MSR_IA32_SPEC_CTRL, rdmsr(MSR_IA32_SPEC_CTRL) | bits) };
}

/// On entry from user mode: write IBRS again where the part has only legacy
/// IBRS, and nothing at all where it has the enhanced kind.
#[inline(always)]
pub(super) fn entry_rewrite() {
    if REWRITE_ON_ENTRY.load(Ordering::Relaxed) {
        ibrs_enable();
    }
}

fn enhanced_ibrs() -> bool {
    // SAFETY: CPUID reported ARCH_CAPABILITIES, so the MSR exists and a read
    // has no side effect.
    cpuid::has_arch_capabilities()
        && unsafe { rdmsr(MSR_IA32_ARCH_CAPABILITIES) } & ARCH_CAP_IBRS_ALL != 0
}
