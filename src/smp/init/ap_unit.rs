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

use super::boot_inputs::ApBootInputs;
use crate::memory::addr::PhysAddr;
use crate::smp::boot_claim::budget_ticks;
use crate::smp::constants::{AP_TRAMPOLINE_ADDR, PERCPU_STACK_SIZE};
use crate::smp::state::CPU_DESCRIPTORS;
use crate::smp::trampoline::{write_per_ap_context, PerApBootContext};
use crate::smp::{CpuDescriptor, CpuState};
use core::sync::atomic::Ordering;

/*
 * Two deadlines, because there are two different questions.
 *
 * Entry: did the AP run at all? A woken AP reaches Rust in microseconds on
 * real hardware. The budget is generous for emulators with many vCPUs, and it
 * is the one that decides whether the trampoline may be reused: past it the
 * boot CPU takes the AP's claim and parks it with INIT.
 *
 * Online: the AP has entered and is setting itself up (GDT, IDT, guard pages,
 * LAPIC timer). That takes locks the boot CPU may hold, so it can be slower,
 * and an AP past entry is never sent INIT: it may be holding one of them.
 */
const AP_ENTRY_TIMEOUT_MS: u64 = 1000;
const AP_ONLINE_TIMEOUT_MS: u64 = 1000;
/// Used when the counter rate is unknown: about a second at up to 5 GHz.
const AP_TIMEOUT_FALLBACK_TICKS: u64 = 5_000_000_000;

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
        Ok(true)
    } else {
        /*
         * Not marked Offline. This AP entered and then missed the deadline;
         * it was not stopped. Writing Offline here raced the AP's own write
         * of Online: lose that race and a running CPU is recorded as down,
         * `shootdown::broadcast` skips it because it filters on
         * `cpu_is_online`, and it goes on holding stale TLB entries with
         * nobody flushing them. Leaving the descriptor in Starting makes the
         * CPU the only writer of its own Online.
         */
        report(cpu_id, apic_id, b" timeout after entry", b"");
        Ok(false)
    }
}

/// Take the AP's claim and park it. False when the AP claimed first, in which
/// case it is running and must be waited for instead.
fn abandon(ap: &CpuDescriptor, apic_id: u32) -> bool {
    if !ap.boot_claim.bsp_abandon() {
        return false;
    }
    // The AP can no longer pass its claim, so nothing else writes this state.
    ap.set_state(CpuState::Offline);
    let _ = crate::arch::x86_64::interrupt::apic::park_ap(apic_id);
    true
}

fn report(cpu_id: usize, apic_id: u32, what: &[u8], detail: &[u8]) {
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[SMP] ap=").dec(cpu_id as u64);
    l.str(b" apic=").dec(apic_id as u64);
    l.str(what);
    l.str(detail);
    l.end();
}

fn configure_descriptor(ap: &CpuDescriptor, cpu_id: usize, apic_id: u32, stack_base: u64) {
    ap.set_identity(cpu_id as u32, apic_id, PERCPU_STACK_SIZE);
    ap.stack_base.store(stack_base, Ordering::Release);
    ap.boot_claim.arm();
    // Published last: the AP, and anything sending it an IPI, must see a
    // complete descriptor before they see it leave Offline.
    ap.set_state(CpuState::Starting);
}

fn write_context(cpu_id: usize, stack_top: u64, boot: &ApBootInputs) -> Result<(), &'static str> {
    let ctx = PerApBootContext::new(boot.pml4_phys, stack_top, boot.entry_ptr, cpu_id as u32);
    write_per_ap_context(PhysAddr::new(AP_TRAMPOLINE_ADDR), &ctx)
        .map_err(|_| "Failed to patch AP trampoline context")
}

fn wait_for(done: impl Fn() -> bool, ms: u64) -> bool {
    let start = super::time::read_tsc();
    let budget =
        budget_ticks(ms, crate::sys::timer::tsc::tsc_frequency(), AP_TIMEOUT_FALLBACK_TICKS);
    while !done() {
        if super::time::read_tsc().wrapping_sub(start) > budget {
            return done();
        }
        core::hint::spin_loop();
    }
    true
}
