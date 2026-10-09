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

//! Hard-IRQ dispatcher. Each broker vector ISR tail-calls `on_vector`, which
//! must not allocate, take locks of its own, or touch IPC or paging. It bumps
//! the per-grant counter, masks an INTx line at the IO-APIC, wakes the slot's
//! armed `MkIrqWait` waiter with a try-only wake, and EOIs the LAPIC.
//! Capsules without an armed waiter see the increment via `MkIrqPoll`.

use core::sync::atomic::Ordering;

use super::grant::NO_LINE;
use super::slots::SLOTS;
use crate::arch::interrupt::broker::slot_of;

const SEQ_SATURATION: u64 = u64::MAX - 1;

#[inline]
pub fn on_vector(vector: u8) {
    if option_env!("NONOS_FBCONSOLE").is_some() {
        // Bring-up diagnostic: flash the on-screen device-IRQ marker so a
        // photo shows whether device interrupts are being delivered at all.
        crate::interrupts::timer::heartbeat::device_irq_blip();
    }
    let slot_idx = match slot_of(vector) {
        Some(i) => i,
        None => {
            crate::interrupts::apic::send_eoi();
            return;
        }
    };
    let slot = &SLOTS[slot_idx];
    if !slot.active.load(Ordering::Acquire) {
        crate::interrupts::apic::send_eoi();
        return;
    }

    // An MSI or MSI-X slot has no pin behind it. Masking GSI 0 for it would
    // silence whatever sits on that pin, and on a board with no GSI 0 every
    // message would be counted as an overflow and never reach its waiter.
    let gsi = slot.gsi.load(Ordering::Relaxed);
    if gsi != NO_LINE && crate::arch::interrupt::ioapic::mask(gsi, true).is_err() {
        slot.overflow.fetch_add(1, Ordering::AcqRel);
        crate::interrupts::apic::send_eoi();
        return;
    }

    let prev = slot.seq.fetch_add(1, Ordering::AcqRel);
    if prev >= SEQ_SATURATION {
        slot.overflow.fetch_add(1, Ordering::AcqRel);
    }

    /* Try only: code this ISR interrupted may hold the waiter's state. */
    let waiter = slot.waiter.swap(0, Ordering::AcqRel);
    if waiter != 0 && !crate::process::scheduler::dispatch::try_wake::try_wake_process(waiter) {
        let _ = slot.waiter.compare_exchange(0, waiter, Ordering::AcqRel, Ordering::Relaxed);
    }

    crate::interrupts::apic::send_eoi();
}
