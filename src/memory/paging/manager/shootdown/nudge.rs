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

//! The second delivery of a slow round: an NMI to every target still marked.
//!
//! The vector is the cheap path and answers almost every round, but it waits
//! for the target to unmask interrupts, and a cpu spinning with interrupts
//! masked on a lock whose holder is waiting for this round never does. An NMI
//! is taken regardless, and the NMI handler serves the round first.

use core::sync::atomic::Ordering;

/// Re-send the round in flight as an NMI to each cpu that has not acked.
///
/// Only the round's targets carry a mark while `SHOOTDOWN_LOCK` is held, so
/// the marks name exactly the cpus still owing. One that acks between the
/// check and the NMI finds nothing pending; the handler knows the NMI was
/// sent on purpose and returns without treating it as a hardware event.
pub(super) fn nudge_outstanding() -> u32 {
    let mut sent = 0u32;
    for cpu in 0..crate::smp::MAX_CPUS {
        if !crate::smp::cpu_is_online(cpu) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if d.tlb_flush_pending.load(Ordering::Acquire) == 0 {
            continue;
        }
        if crate::smp::nmi::kick(cpu, d.apic_id) {
            sent += 1;
        }
    }
    sent
}
