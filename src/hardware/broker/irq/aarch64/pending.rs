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

use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

// 128 GIC bindings per kernel; the trap path's linear scan stays cheap.
pub(super) const MAX_GRANTS: usize = 128;

// `intid == 0` marks a free slot: SGIs are kernel-owned and the bind
// path rejects intid < 32, so no real binding is shadowed.
#[repr(C, align(64))]
pub(super) struct Entry {
    pub(super) intid: AtomicU32,
    pub(super) pid: AtomicU32,
    pub(super) grant_id: AtomicU64,
    pub(super) device_id: AtomicU64,
    pub(super) claim_epoch: AtomicU64,
    pub(super) pending: AtomicU64,
    pub(super) overflow: AtomicU64,
    pub(super) waiter: AtomicU32, // the pid in `MkIrqWait`, zero for none
}

pub(super) static SLOTS: [Entry; MAX_GRANTS] = [const {
    Entry {
        intid: AtomicU32::new(0),
        pid: AtomicU32::new(0),
        grant_id: AtomicU64::new(0),
        device_id: AtomicU64::new(0),
        claim_epoch: AtomicU64::new(0),
        pending: AtomicU64::new(0),
        overflow: AtomicU64::new(0),
        waiter: AtomicU32::new(0),
    }
}; MAX_GRANTS];

static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

// CAS-claim a free slot and grant id; None when full or `intid` is bound.
pub(super) fn alloc(
    intid: u32,
    pid: u32,
    device_id: u64,
    claim_epoch: u64,
) -> Option<(&'static Entry, u64)> {
    if super::pending_find::find_by_intid(intid).is_some() {
        return None;
    }
    for e in SLOTS.iter() {
        if e.intid.compare_exchange(0, intid, Ordering::AcqRel, Ordering::Acquire).is_ok() {
            let id = NEXT_GRANT_ID.fetch_add(1, Ordering::Relaxed);
            e.pid.store(pid, Ordering::Release);
            e.grant_id.store(id, Ordering::Release);
            e.device_id.store(device_id, Ordering::Release);
            e.claim_epoch.store(claim_epoch, Ordering::Release);
            e.pending.store(0, Ordering::Release);
            e.overflow.store(0, Ordering::Release);
            e.waiter.store(0, Ordering::Release);
            return Some((e, id));
        }
    }
    None
}
