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

/// The APIC id of the CPU that calls this, read from that CPU's own hardware.
///
/// It is deliberately not `CACHED_ID`. That cache is one global word, written
/// by whichever CPU initialised the APIC, so every secondary asking which one
/// it was got the boot CPU's answer. `percpu::init_ap` populates `apic_id`
/// from here, so every AP recorded 0, and the TLB shootdown then addressed its
/// IPIs by that field: three rounds all sent to the boot CPU, none to the CPUs
/// that owed the acknowledgement, and the originator spun to its deadline and
/// halted the machine.
///
/// Before the APIC is initialised there is exactly one CPU executing and the
/// cache holds its id, which is why that path is still safe to take. It also
/// has to be taken: the MMIO read below dereferences a base that does not
/// exist yet.
pub fn id() -> u32 {
    if INITIALIZED.load(Ordering::Acquire) {
        read_id_internal()
    } else {
        CACHED_ID.load(Ordering::Acquire)
    }
}

pub fn read_id_internal() -> u32 {
    if X2APIC_MODE.load(Ordering::Acquire) {
        (rdmsr(IA32_X2APIC_APICID) & 0xFFFF_FFFF) as u32
    } else {
        (mmio_r32(LAPIC_ID) >> 24) & 0xFF
    }
}

pub fn set_tpr(priority: u8) {
    CURRENT_TPR.store(priority, Ordering::Release);
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_TPR, priority as u64);
    } else {
        mmio_w32(LAPIC_TPR, priority as u32);
    }
}

pub fn get_tpr() -> u8 {
    CURRENT_TPR.load(Ordering::Acquire)
}

#[inline(always)]
pub fn eoi() {
    crate::process::accounting::bump_total(crate::process::accounting::Total::Interrupts);
    if X2APIC_MODE.load(Ordering::Acquire) {
        wrmsr(IA32_X2APIC_EOI, 0);
    } else {
        mmio_w32(LAPIC_EOI, 0);
    }
}

pub fn send_eoi() {
    eoi();
}

pub fn version() -> u32 {
    if X2APIC_MODE.load(Ordering::Acquire) {
        (rdmsr(0x803) & 0xFF) as u32
    } else {
        mmio_r32(LAPIC_VER) & 0xFF
    }
}

pub fn max_lvt() -> u8 {
    if X2APIC_MODE.load(Ordering::Acquire) {
        ((rdmsr(0x803) >> 16) & 0xFF) as u8
    } else {
        ((mmio_r32(LAPIC_VER) >> 16) & 0xFF) as u8
    }
}
