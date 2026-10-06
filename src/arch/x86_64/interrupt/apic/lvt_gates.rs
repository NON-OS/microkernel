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

//! IDT gates for the two vectors the local APIC raises on its own: spurious
//! (SVR, 0xFF) and error (LVT error, 0xFE).
//!
//! The spurious vector had no gate. A real APIC raises it whenever an
//! interrupt it was about to deliver is withdrawn before the CPU acknowledges
//! it, which on hardware is a matter of time (a level line dropping while
//! interrupts are masked is enough); an emulator almost never does. Delivered
//! to a not-present gate it is a segment-not-present fault in whatever was
//! running. The handler does nothing and sends no EOI: a spurious interrupt
//! sets no in-service bit, and an EOI would retire someone else's (SDM 10.9).

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

use crate::interrupts::idt::vectors::{VECTOR_APIC_ERROR, VECTOR_APIC_SPURIOUS};

/// How many spurious and error interrupts any CPU has taken, and the last ESR
/// value an error left. Read by diagnostics; nothing depends on them.
pub(crate) static SPURIOUS_COUNT: AtomicU64 = AtomicU64::new(0);
pub(crate) static ERROR_COUNT: AtomicU64 = AtomicU64::new(0);
pub(crate) static LAST_ESR: AtomicU32 = AtomicU32::new(0);

extern "x86-interrupt" fn spurious(_frame: InterruptStackFrame) {
    SPURIOUS_COUNT.fetch_add(1, Ordering::Relaxed);
}

extern "x86-interrupt" fn apic_error(_frame: InterruptStackFrame) {
    LAST_ESR.store(crate::sys::apic::ack_error(), Ordering::Relaxed);
    ERROR_COUNT.fetch_add(1, Ordering::Relaxed);
    crate::sys::apic::eoi();
}

pub fn install_gates(idt: &mut InterruptDescriptorTable) {
    idt[VECTOR_APIC_SPURIOUS as usize].set_handler_fn(spurious);
    idt[VECTOR_APIC_ERROR as usize].set_handler_fn(apic_error);
}
