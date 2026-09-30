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

//! Interrupt commands sent with NMI delivery.
//!
//! A fixed vector waits for the target to unmask interrupts; an NMI does
//! not. Two things need a cpu that may be spinning with interrupts masked:
//! a TLB shootdown round that a target has not answered, and the stop that
//! a fatal halt sends every other cpu. The vector field is ignored for this
//! delivery mode and is sent as zero.

use super::constants::*;
use super::ipi_basic::wait_icr_idle;
use super::mmio::mmio_w32;
use super::state::*;
use core::sync::atomic::Ordering;

/// Delivery mode 100b in ICR bits 10:8.
const ICR_DELIV_NMI: u64 = 0x4 << 8;
const ICR_NMI: u64 = ICR_DELIV_NMI | ICR_LEVEL_ASSERT | ICR_TRIG_EDGE;

/// Raise an NMI on the cpu with `apic_id`. `false` means the local APIC is
/// not up yet and nothing was sent.
pub fn nmi_one(apic_id: u32) -> bool {
    if !INITIALIZED.load(Ordering::Acquire) {
        return false;
    }
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, (apic_id as u64) << 32 | ICR_NMI | ICR_DST_PHYSICAL | ICR_SH_NONE);
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, apic_id << 24);
        mmio_w32(LAPIC_ICR_LOW, (ICR_NMI | ICR_DST_PHYSICAL | ICR_SH_NONE) as u32);
    }
    true
}

/// Raise an NMI on every cpu but this one. `false` as for `nmi_one`.
pub fn nmi_others() -> bool {
    if !INITIALIZED.load(Ordering::Acquire) {
        return false;
    }
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, ICR_NMI | ICR_SH_OTHERS);
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, 0);
        mmio_w32(LAPIC_ICR_LOW, (ICR_NMI | ICR_SH_OTHERS) as u32);
    }
    true
}
