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

// x2APIC register access for the local-APIC surface, and the choice of mode.
// In x2APIC mode the MMIO window is disabled and the registers are
// model-specific registers: MSR = 0x800 + (offset >> 4).
//
// The mode is the firmware's unless the machine forces another. Real UEFI
// firmware frequently hands off in x2APIC already, and that is kept: leaving
// x2APIC would need a pass through the disabled state, and a part with an id
// above 0xFE needs x2APIC anyway. The one switch made here is the other way:
// firmware left xAPIC but the MADT lists an id xAPIC cannot address, and the
// part supports x2APIC. Then the boot CPU moves to x2APIC (a legal direct
// transition, SDM 10.12.5) so every CPU can be reached.

use core::sync::atomic::Ordering;

use crate::arch::x86_64::boot::cpu_ops::{cpuid, rdmsr, wrmsr};
use crate::arch::x86_64::interrupt::apic::plan;

use super::state::{LAPIC_PHYS, LAPIC_X2};

const IA32_APIC_BASE: u32 = 0x1B;
const X2APIC_MSR_BASE: u32 = 0x800;

// Latch the mode and the register page, and make sure the APIC is globally
// enabled before any register access. Bit 11 of IA32_APIC_BASE is the
// hardware global-enable: with it clear, both the xAPIC MMIO window and the
// x2APIC MSRs are dead and every LAPIC write (SVR, timer LVT, EOI) silently
// vanishes, so no interrupt is ever delivered. QEMU hands off with it set;
// some real firmware does not, and leaving it clear is exactly the "boots but
// the timer never ticks" symptom.
pub(in crate::sys::apic) fn detect_mode() {
    let mut base = unsafe { rdmsr(IA32_APIC_BASE) };
    if base & plan::BASE_ENABLE == 0 {
        base = (base & !plan::BASE_EXTD) | plan::BASE_ENABLE;
        unsafe { wrmsr(IA32_APIC_BASE, base) };
    }
    let mut x2 = base & plan::BASE_EXTD != 0;
    if !x2 && cpu_has_x2apic() && madt_requires_x2apic() {
        base |= plan::BASE_EXTD;
        unsafe { wrmsr(IA32_APIC_BASE, base) };
        x2 = true;
    }
    LAPIC_PHYS.store(plan::base_phys(base), Ordering::Release);
    LAPIC_X2.store(x2, Ordering::Release);
}

/// Bring an AP's own APIC into the mode the boot CPU chose. INIT leaves the
/// mode as firmware set it, which need not match, and the only legal way out
/// of x2APIC is through disabled; `plan::base_transition` gives the writes.
pub(in crate::sys::apic) fn enter_mode_on_ap() {
    let want = LAPIC_X2.load(Ordering::Acquire);
    let current = unsafe { rdmsr(IA32_APIC_BASE) };
    let (writes, n) = plan::base_transition(current, want);
    for value in &writes[..n] {
        unsafe { wrmsr(IA32_APIC_BASE, *value) };
    }
}

fn cpu_has_x2apic() -> bool {
    cpuid(1).2 & (1 << 21) != 0
}

fn madt_requires_x2apic() -> bool {
    let ids: alloc::vec::Vec<u32> = crate::arch::x86_64::acpi::processors()
        .iter()
        .filter(|p| p.enabled)
        .map(|p| p.apic_id)
        .collect();
    plan::requires_x2apic(&ids)
}

pub(in crate::sys::apic) fn is_x2(reg_order: Ordering) -> bool {
    LAPIC_X2.load(reg_order)
}

pub(in crate::sys::apic) fn read(reg: u32) -> u32 {
    unsafe { rdmsr(X2APIC_MSR_BASE + (reg >> 4)) as u32 }
}

pub(in crate::sys::apic) fn write(reg: u32, value: u32) {
    unsafe { wrmsr(X2APIC_MSR_BASE + (reg >> 4), value as u64) }
}
