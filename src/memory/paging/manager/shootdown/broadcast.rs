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

use super::handle::serve_for;
use super::request::{REQ_PAGES, REQ_PENDING_ACKS, REQ_VA, SHOOTDOWN_LOCK};
use super::select::select;
use super::send::mark_and_send;
use super::wait::wait_for_acks;
use crate::memory::addr::VirtAddr;

/// What one round reached, for `trace`; a kernel built without it keeps none.
#[cfg(feature = "nonos-shootdown-trace")]
pub(super) struct Round {
    pub(super) asid: u32,
    pub(super) pages: u32,
    pub(super) selected: [u64; super::select::WORDS],
    pub(super) targets: u32,
    pub(super) starting: u32,
    pub(super) answered_for: u32,
}

#[cfg(not(feature = "nonos-shootdown-trace"))]
type Round = ();

#[cfg(feature = "nonos-shootdown-trace")]
fn summary(
    asid: u32,
    pages: u32,
    selected: [u64; super::select::WORDS],
    targets: u32,
    starting: u32,
    answered_for: u32,
) -> Round {
    Round { asid, pages, selected, targets, starting, answered_for }
}

#[cfg(not(feature = "nonos-shootdown-trace"))]
fn summary(_: u32, _: u32, _: [u64; super::select::WORDS], _: u32, _: u32, _: u32) -> Round {}

pub(super) fn broadcast(va: VirtAddr, page_count: u32, asid: u32) {
    /*
     * The whole round runs masked. A holder of `SHOOTDOWN_LOCK` that took an
     * interrupt could be switched away, or could reach a flush from the
     * handler, and every other cpu, this one's next thread included, would
     * spin on the lock with interrupts masked waiting for a round that no
     * longer runs. The wait itself serves this cpu's own requests by hand,
     * so masking costs it nothing it needs to hear.
     */
    let done = crate::arch::run_without_interrupts(|| round(va, page_count, asid));
    // After the round, its lock released: the line takes the serial lock, and
    // no cpu may wait on this one for an acknowledgement while it does.
    #[cfg(feature = "nonos-shootdown-trace")]
    if let Some(done) = &done {
        super::trace::report(done);
    }
    #[cfg(not(feature = "nonos-shootdown-trace"))]
    let _ = done;
}

fn round(va: VirtAddr, page_count: u32, asid: u32) -> Option<Round> {
    let me = crate::smp::percpu::current();
    // Serve any round already in flight while waiting for our turn. Page-table
    // mutation sites reach here with interrupts masked, so a cpu that simply
    // blocked on the lock could not answer the holder's IPI, and the two would
    // wait on each other until the timeout below halted the machine.
    let _guard = loop {
        if let Some(guard) = SHOOTDOWN_LOCK.try_lock() {
            break guard;
        }
        serve_for(me);
        core::hint::spin_loop();
    };

    /*
     * Paired with the fence a cpu takes between recording its asid and loading
     * CR3 (`switch_address_space`). The page table writes this round flushes
     * are already done; either that cpu's asid is seen below, or its CR3 load
     * comes after those writes and it cannot have cached the old entries.
     */
    core::sync::atomic::fence(Ordering::SeqCst);
    let (selected, targets, starting) = select(asid);
    if targets == 0 {
        return None;
    }

    REQ_VA.store(va.as_u64(), Ordering::Release);
    REQ_PAGES.store(page_count, Ordering::Release);
    REQ_PENDING_ACKS.store(targets, Ordering::SeqCst);

    mark_and_send(&selected);
    let answered_for = wait_for_acks(&selected);
    Some(summary(asid, page_count, selected, targets, starting, answered_for))
}
