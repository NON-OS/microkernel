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

//! The per-CPU register programming every local APIC gets, boot CPU and APs
//! alike. Each CPU has its own APIC behind the same address, so each runs
//! this for itself; only the mode and the register page are shared.

use core::sync::atomic::Ordering;

use crate::arch::x86_64::interrupt::apic::plan::{self, LintNmi};

use super::constants::{
    ERROR_VECTOR, LAPIC_ESR, LAPIC_ID, LAPIC_LVT_CMCI, LAPIC_LVT_ERROR, LAPIC_LVT_LINT0,
    LAPIC_LVT_LINT1, LAPIC_LVT_PERF, LAPIC_LVT_THERMAL, LAPIC_LVT_TIMER, LAPIC_SVR, LAPIC_TPR,
    LAPIC_VERSION, LVT_MASKED, SPURIOUS_VECTOR, SVR_APIC_ENABLE,
};
use super::regs::{lapic_read_raw, lapic_write_raw};
use super::state::{LAPIC_PHYS, LAPIC_X2};

/// Program this CPU's APIC into the state the kernel runs it in.
///
/// SVR: software enable, spurious vector 0xFF, and no directed-EOI
/// suppression. Suppression would stop a level-triggered EOI from reaching
/// the I/O APIC, and nothing here writes the I/O APIC's own EOI register, so
/// the first level interrupt on a CPU would leave its line stuck for good.
///
/// Every LVT is masked except the error entry, which gets its own vector so
/// an APIC error is counted and acknowledged instead of arriving on a vector
/// with no gate. LINT0 and LINT1 follow the MADT NMI entries for this CPU
/// (`plan::lint_lvts`): LINT0 carries the 8259's INTR, which must never reach
/// a CPU on an I/O APIC system, and an AP that took it as an NMI, as the AP
/// path used to program, would turn a stray PIC interrupt into an NMI.
///
/// # Safety
/// The APIC must be globally enabled in the mode `LAPIC_X2` names, and in
/// xAPIC mode `LAPIC_BASE` must map the register page.
pub(in crate::sys::apic) unsafe fn program_local_vectors(is_bsp: bool) {
    unsafe {
        lapic_write_raw(LAPIC_SVR, SVR_APIC_ENABLE | SPURIOUS_VECTOR);
        lapic_write_raw(LAPIC_TPR, 0);

        let max_lvt = (lapic_read_raw(LAPIC_VERSION) >> 16) & 0xFF;
        lapic_write_raw(LAPIC_LVT_TIMER, LVT_MASKED);
        if max_lvt >= 4 {
            lapic_write_raw(LAPIC_LVT_PERF, LVT_MASKED);
        }
        if max_lvt >= 5 {
            lapic_write_raw(LAPIC_LVT_THERMAL, LVT_MASKED);
        }
        if max_lvt >= 6 {
            lapic_write_raw(LAPIC_LVT_CMCI, LVT_MASKED);
        }

        let (lint0, lint1) = lint_for_this_cpu(own_id(), is_bsp);
        lapic_write_raw(LAPIC_LVT_LINT0, lint0);
        lapic_write_raw(LAPIC_LVT_LINT1, lint1);

        // Clear anything latched before the error vector is opened. The
        // register must be written before it is read; twice clears both the
        // latched and the pending set.
        lapic_write_raw(LAPIC_ESR, 0);
        lapic_write_raw(LAPIC_ESR, 0);
        lapic_write_raw(LAPIC_LVT_ERROR, ERROR_VECTOR);
    }
}

/// Bring up this AP's own APIC: same mode as the boot CPU, then the same
/// programming. Interrupts must be masked.
pub fn init_ap_local_apic() {
    super::x2apic::enter_mode_on_ap();
    // SAFETY: the mode was just made to match `LAPIC_X2`, and the register
    // page the boot CPU mapped is the same physical page on every CPU.
    unsafe { program_local_vectors(false) };
}

/// Acknowledge an APIC error interrupt: latch the ESR and return what it
/// held. Called from the error vector's handler on the CPU that raised it.
pub fn ack_error() -> u32 {
    // SAFETY: this CPU's own ESR; the write latches, the read reports.
    unsafe {
        lapic_write_raw(LAPIC_ESR, 0);
        lapic_read_raw(LAPIC_ESR)
    }
}

/// Physical address of the xAPIC register page as IA32_APIC_BASE gave it.
pub fn lapic_phys_base() -> u64 {
    LAPIC_PHYS.load(Ordering::Acquire)
}

fn own_id() -> u32 {
    // SAFETY: a read of this CPU's own ID register.
    let raw = unsafe { lapic_read_raw(LAPIC_ID) };
    if LAPIC_X2.load(Ordering::Acquire) {
        raw
    } else {
        raw >> 24
    }
}

fn lint_for_this_cpu(apic_id: u32, is_bsp: bool) -> (u32, u32) {
    let uid = crate::arch::x86_64::acpi::processors()
        .iter()
        .find(|p| p.apic_id == apic_id)
        .map(|p| p.processor_uid);
    let nmis: alloc::vec::Vec<LintNmi> = crate::arch::x86_64::acpi::nmi_configs()
        .iter()
        .map(|n| LintNmi { processor_uid: n.processor_uid, lint: n.lint, flags: n.flags })
        .collect();
    plan::lint_lvts(&nmis, uid, is_bsp)
}
