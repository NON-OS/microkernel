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

use core::sync::atomic::Ordering;

use crate::arch::interrupt_controller::Ipi;

/*
 * Mark and send from the set `select` chose rather than re-deriving it, and
 * only once `broadcast` has published the request and the ack count. A cpu serves
 * this round by hand the moment it sees its own mark, from the lock spin
 * in `broadcast` or from `lock_responsive`, with no ipi involved; marking before
 * the count was armed let that cpu pay an ack into a count of zero, which
 * wrapped and was then overwritten by the arming store, so the ack was
 * owed by nobody and the wait below always reached its deadline. Deriving
 * the set twice would be its own bug: a cpu that came online in between
 * would be marked without being counted.
 */
pub(super) fn mark_and_send(selected: &[u64]) {
    for cpu in 0..crate::smp::MAX_CPUS {
        if selected[cpu / 64] & (1u64 << (cpu % 64)) == 0 {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        d.tlb_flush_pending.store(1, Ordering::Release);
        /*
         * An x2APIC ICR write is a WRMSR, which does not wait for earlier
         * stores to drain. Without a full fence the vector can arrive before
         * the mark is visible, the handler finds nothing pending, and a cpu
         * idling with interrupts on never looks again.
         */
        core::sync::atomic::fence(Ordering::SeqCst);
        let _ = crate::arch::interrupt_controller::send_ipi(d.apic_id, Ipi::TlbShootdown);
    }
}
