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

//! Each CPU's interrupt in service, so it is finished exactly once.
//!
//! The dispatcher finishes an interrupt when its handler returns, but an IPI
//! that halts the CPU never returns, and the rest finish early so the sender
//! is answered before their work runs on. Before this slot existed the early
//! ones were finished twice, and a second EOI with nothing active drops
//! whatever priority happens to be running.

use core::sync::atomic::{AtomicU32, Ordering};

use super::acked::Acked;
use crate::arch::aarch64::boot::stack::MAX_CPUS;
use crate::arch::aarch64::cpu::cpu_id;

const IDLE: u32 = u32::MAX;

// Only the owning CPU touches its slot, with interrupts masked, so the
// accesses need atomicity for the type system, not ordering.
static SLOTS: [AtomicU32; MAX_CPUS] = [const { AtomicU32::new(IDLE) }; MAX_CPUS];

fn slot() -> &'static AtomicU32 {
    &SLOTS[cpu_id()]
}

/// Mark `acked` as in service and return what was there before, which is
/// handed back to `leave` once the interrupt is done.
pub fn enter(acked: Acked) -> u32 {
    slot().swap(acked.raw, Ordering::Relaxed)
}

/// Finish the interrupt this CPU is serving, if it is not finished already.
pub fn complete() {
    let raw = slot().swap(IDLE, Ordering::Relaxed);
    if raw != IDLE {
        super::serve::end(raw);
    }
}

/// Restore the slot `enter` replaced.
pub fn leave(outer: u32) {
    slot().store(outer, Ordering::Relaxed);
}
