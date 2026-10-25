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

//! The boot CPU's APIC id, latched before its SMP descriptor is filled.
//!
//! The descriptor used to take `CACHED_ID`, which `init_runtime` fills well
//! after `smp::init_bsp` and which reads 0 until then. On a machine whose
//! boot CPU is not APIC 0, descriptor 0 named a CPU that does not exist and
//! the boot CPU's first `cpu_id()` found no descriptor and halted.

use core::sync::atomic::Ordering;

use super::boot_id_rule::reconcile;
use super::state::CACHED_ID;

/// The id `cpu_id()` looks the boot CPU up by, checked against what its local
/// APIC answers to and published for interrupt routing.
pub fn latch_boot_id() -> Result<u32, &'static str> {
    // CPUID, leaf 0xB when the part has it: what `cpu_id()` resolves by.
    let cpuid = crate::arch::cpu::get_cpu_id();
    let lapic = crate::sys::apic::local_apic_id();
    let x2apic = crate::sys::apic::lapic_state().is_some_and(|(x2, _)| x2);
    match reconcile(cpuid, lapic, x2apic) {
        Ok(id) => {
            // Device interrupts are aimed here; until now it said 0, which
            // only reached the boot CPU where the boot CPU was APIC 0.
            CACHED_ID.store(id, Ordering::Release);
            Ok(id)
        }
        Err((cpuid, lapic)) => {
            crate::log::logger::log_info!(
                "[APIC] boot CPU: CPUID id {} but local APIC id {}",
                cpuid,
                lapic
            );
            Err("boot CPU's CPUID and local APIC ids differ")
        }
    }
}
