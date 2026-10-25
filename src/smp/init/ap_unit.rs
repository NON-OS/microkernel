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
mod abandon;
mod park;
mod prepare;
mod report;
mod wait;

use super::boot_inputs::ApBootInputs;
use crate::smp::constants::{AP_TRAMPOLINE_ADDR, PERCPU_STACK_SIZE};
use crate::smp::state::CPU_DESCRIPTORS;
use crate::smp::CpuState;
use abandon::abandon;
use core::sync::atomic::Ordering;
use park::park;
use prepare::{configure_descriptor, write_context};
use report::report;
use wait::{wait_for, AP_ENTRY_TIMEOUT_MS, AP_ONLINE_TIMEOUT_MS};

pub(super) fn start(
    cpu_id: usize,
    apic_id: u32,
    boot: &ApBootInputs,
) -> Result<bool, &'static str> {
    let stack_base = super::stack::allocate(cpu_id)?;
    let stack_top = stack_base + PERCPU_STACK_SIZE as u64;
    let ap = &CPU_DESCRIPTORS[cpu_id];

    configure_descriptor(ap, cpu_id, apic_id, stack_base);
    write_context(cpu_id, stack_top, boot)?;
    /*
     * `Starting` must be visible before INIT can land. A shootdown that reads
     * this cpu as `Offline` skips it on the strength of INIT emptying its TLB
     * after that round's page-table write (`shootdown::select`); were the
     * store still buffered when INIT went out, the AP could cache an entry
     * the round then never flushes.
     */
    core::sync::atomic::fence(Ordering::SeqCst);
    if let Err(e) =
        crate::arch::x86_64::interrupt::apic::start_ap(apic_id, (AP_TRAMPOLINE_ADDR >> 12) as u8)
    {
        // Nothing may have reached it, or a STARTUP may have. Park it either
        // way so the trampoline can be reused, and say why.
        report(cpu_id, apic_id, b" startup failed: ", e.as_str().as_bytes());
        if abandon(ap, apic_id) {
            return Ok(false);
        }
        // It entered regardless, so it is running; wait for it below.
    }

    if !wait_for(|| ap.boot_claim.entered(), AP_ENTRY_TIMEOUT_MS) && abandon(ap, apic_id) {
        report(cpu_id, apic_id, b" no response, parked", b"");
        return Ok(false);
    }

    if wait_for(|| ap.state() == CpuState::Online, AP_ONLINE_TIMEOUT_MS) {
        report(cpu_id, apic_id, b" online", b"");
        return Ok(true);
    }
    report(cpu_id, apic_id, b" timeout after entry", b"");
    Ok(park(ap, cpu_id, apic_id))
}
