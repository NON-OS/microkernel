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

//! A copy-on-write fault that another CPU already resolved.

use core::sync::atomic::{AtomicU64, Ordering};

use super::super::core::PagingManager;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::{pte_is_user, pte_is_writable};
use crate::smp::MAX_CPUS;

impl PagingManager {
    /*
     * The fault reported a read-only entry when it was taken, not now.
     * Two threads of one process on two CPUs can write the same
     * copy-on-write page at once; the second waits on the manager lock
     * while the first copies it. The record `handle_cow_fault` reads then
     * no longer says copy-on-write, and the second thread was killed for a
     * protection fault on a page it may write. The caller holds the lock, so
     * the entry cannot change before its copy; writable means the faulting
     * access is retried, once (see `retry_once`).
     */
    pub(super) fn cow_raced(&self, virtual_addr: VirtAddr, page_addr: u64) -> bool {
        let writable =
            self.leaf_entry(virtual_addr).is_ok_and(|e| pte_is_writable(e) && pte_is_user(e));
        retry_once(page_addr, writable)
    }
}

static RETRIED: [AtomicU64; MAX_CPUS] = [const { AtomicU64::new(0) }; MAX_CPUS];

/*
 * Whether to send the access back to run again. A write that faults on an
 * entry already writable was racing another CPU's copy and succeeds when run
 * again. One that faults a second time on the same page is not such a race
 * (a supervisor write the access-prevention bit refused, say) and is failed
 * as before rather than retried for ever. A lone CPU cannot race itself, so
 * the single-CPU image fails every such fault as it always did.
 */
fn retry_once(page_addr: u64, writable: bool) -> bool {
    if !cfg!(feature = "nonos-smp") {
        return false;
    }
    let slot = &RETRIED[crate::smp::cpu_id() % MAX_CPUS];
    if writable && slot.swap(page_addr, Ordering::Relaxed) != page_addr {
        return true;
    }
    slot.store(0, Ordering::Relaxed);
    false
}
