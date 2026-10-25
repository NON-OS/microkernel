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

//! One round, under `SHOOTDOWN_LOCK` with interrupts masked: select the
//! targets, publish the request, mark and send, and wait for every ack.

use core::sync::atomic::Ordering;

use super::handle::serve_for;
use super::request::{REQ_PAGES, REQ_PENDING_ACKS, REQ_VA, SHOOTDOWN_LOCK};
use super::select::select;
use super::send::mark_and_send;
use super::summary::{summary, Round};
use super::wait::wait_for_acks;
use crate::memory::addr::VirtAddr;

pub(super) fn round(va: VirtAddr, page_count: u32, asid: u32) -> Option<Round> {
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
