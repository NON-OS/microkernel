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

use super::constants::*;
use super::mmio::{mmio_r32, mmio_w32};
use super::state::*;
use core::sync::atomic::Ordering;

/*
 * Every interrupt command written below carries it. The bit is level-assert,
 * and the processor delivers a command without it only in the one case it
 * still means something, an INIT level de-assert; a fixed-delivery command
 * with the bit clear is dropped. Of the mode constants this file ORs together
 * it is also the only one that is not zero, so leaving it out produced a word
 * that was purely destination and vector and looked complete while sending
 * nothing. `ipi_ap.rs` passes it by hand, which is why AP startup worked while
 * every runtime IPI, including the TLB shootdown, silently went nowhere.
 */
const ICR_SEND: u64 = ICR_DELIV_FIXED | ICR_LEVEL_ASSERT | ICR_TRIG_EDGE;

pub fn ipi_self(vec: u8) {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, ICR_SEND | ICR_SH_SELF | (vec as u64));
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, 0);
        mmio_w32(LAPIC_ICR_LOW, (ICR_SEND | ICR_SH_SELF) as u32 | vec as u32);
    }
}

pub fn ipi_one(apic_id: u32, vec: u8) {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(
            IA32_X2APIC_ICR,
            (apic_id as u64) << 32
                | ICR_SEND
                | ICR_DST_PHYSICAL
                | ICR_SH_NONE
                | (vec as u64),
        );
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, apic_id << 24);
        mmio_w32(LAPIC_ICR_LOW, ICR_SEND as u32 | vec as u32);
    }
}

pub fn ipi_all(vec: u8) {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, ICR_SEND | ICR_SH_ALL | (vec as u64));
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, 0);
        mmio_w32(LAPIC_ICR_LOW, (ICR_SEND | ICR_SH_ALL) as u32 | vec as u32);
    }
}

pub fn ipi_others(vec: u8) {
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_ICR, ICR_SEND | ICR_SH_OTHERS | (vec as u64));
    } else {
        wait_icr_idle();
        mmio_w32(LAPIC_ICR_HIGH, 0);
        mmio_w32(LAPIC_ICR_LOW, (ICR_SEND | ICR_SH_OTHERS) as u32 | vec as u32);
    }
}

pub(super) fn wait_icr_idle() {
    for _ in 0..100_000 {
        if (mmio_r32(LAPIC_ICR_LOW) & ICR_BUSY) == 0 {
            return;
        }
        core::hint::spin_loop();
    }
}
