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

//! The waiter's acknowledgement on behalf of a target that will never answer.

use core::sync::atomic::Ordering;

use super::request::REQ_PENDING_ACKS;

/// Acknowledge for each target the boot CPU gave up on after `select` read it
/// as `Starting`: one that never claimed its start, or one that claimed,
/// parked at a checkpoint and was reset (`ap_unit::park`). Either way INIT
/// emptied its TLB, and it will never answer; without this the round waits to
/// its deadline and stops the machine. Taking the mark is what makes exactly
/// one side acknowledge: the cpu itself, had it got there, would have taken it
/// first. Returns how many it paid.
pub(super) fn answer_for_abandoned(selected: &[u64]) -> u32 {
    let mut answered = 0;
    for cpu in 0..crate::smp::MAX_CPUS {
        if selected[cpu / 64] & (1u64 << (cpu % 64)) == 0 || !crate::smp::cpu_is_offline(cpu) {
            continue;
        }
        let Some(d) = crate::smp::percpu::get(cpu) else {
            continue;
        };
        if d.tlb_flush_pending.swap(0, Ordering::AcqRel) != 0 {
            REQ_PENDING_ACKS.fetch_sub(1, Ordering::Release);
            answered += 1;
        }
    }
    answered
}
