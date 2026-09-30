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

use super::request::{REQ_PAGES, REQ_PENDING_ACKS, REQ_VA};
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::tlb;

/// Flush for the round in progress, if this cpu is one of its targets.
///
/// Driven by the TlbShootdown vector, and also called directly by a cpu
/// spinning for the lock in `broadcast`. The pending flag makes it safe either
/// way: it is what says the round applies to us, and clearing it before the
/// ack means neither path can acknowledge twice.
///
/// Never halts: a CPU that cannot name itself (not yet registered) returns
/// without serving. It was not among the round's targets, which are chosen
/// from registered CPUs, so no acknowledgement is owed.
///
/// Also the first thing the NMI handler runs, so it stays NMI-safe: the cpu is
/// named by its APIC id (cpuid and a scan of atomics), never through GS, which
/// an NMI in the kernel-entry window finds still holding the user base; it
/// takes no lock, allocates nothing and prints nothing. Interrupting itself is
/// harmless, since whichever call clears the pending flag is the one that acks.
/// Returns whether this call acknowledged a round.
pub fn handle_shootdown_ipi() -> bool {
    match crate::smp::percpu::try_current() {
        Some(me) => serve_for(me),
        None => false,
    }
}

/// Whether any round is waiting for acknowledgements.
///
/// A poll that sees `false` may skip resolving its cpu: a round arms its ack
/// count before it marks any target, so a cpu marked for a round the poll
/// missed is found by the next poll, and the vector covers the rest.
#[inline]
pub fn shootdown_in_flight() -> bool {
    REQ_PENDING_ACKS.load(Ordering::Acquire) != 0
}

/// `handle_shootdown_ipi` for a cpu already resolved, so a loop that polls
/// does not pay the interrupt-controller read that names the cpu every time.
pub(super) fn serve_for(me: &crate::smp::percpu::PerCpuData) -> bool {
    if me.tlb_flush_pending.swap(0, Ordering::AcqRel) == 0 {
        return false;
    }
    let pages = REQ_PAGES.load(Ordering::Acquire);
    if pages == 0 {
        tlb::invalidate_all();
    } else {
        let base = VirtAddr::new(REQ_VA.load(Ordering::Acquire));
        for i in 0..pages as usize {
            let va = VirtAddr::new(base.as_u64() + (i * PAGE_SIZE_4K) as u64);
            tlb::invalidate_page(va);
        }
    }
    REQ_PENDING_ACKS.fetch_sub(1, Ordering::Release);
    true
}
