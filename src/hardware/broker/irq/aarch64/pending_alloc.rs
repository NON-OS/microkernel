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

use core::sync::atomic::{AtomicU64, Ordering};

use super::pending::{Entry, SLOTS};

static NEXT_GRANT_ID: AtomicU64 = AtomicU64::new(1);

/// A line's slot and its grant id; `None` when full or `intid` is bound.
pub(super) fn alloc(
    intid: u32,
    pid: u32,
    device_id: u64,
    claim_epoch: u64,
) -> Option<(&'static Entry, u64)> {
    let id = grant_ids(1);
    claim(intid, id, pid, device_id, claim_epoch, false).map(|e| (e, id))
}

/// `count` consecutive grant ids, the first returned.
pub(super) fn grant_ids(count: u64) -> u64 {
    NEXT_GRANT_ID.fetch_add(count, Ordering::Relaxed)
}

/// CAS-claim a free slot for `intid` under grant `id`.
pub(super) fn claim(
    intid: u32,
    id: u64,
    pid: u32,
    device_id: u64,
    epoch: u64,
    message: bool,
) -> Option<&'static Entry> {
    if super::pending_find::find_by_intid(intid).is_some() {
        return None;
    }
    let e = SLOTS.iter().find(|e| {
        e.intid.compare_exchange(0, intid, Ordering::AcqRel, Ordering::Acquire).is_ok()
    })?;
    e.pid.store(pid, Ordering::Release);
    e.grant_id.store(id, Ordering::Release);
    e.device_id.store(device_id, Ordering::Release);
    e.claim_epoch.store(epoch, Ordering::Release);
    e.pending.store(0, Ordering::Release);
    e.overflow.store(0, Ordering::Release);
    e.waiter.store(0, Ordering::Release);
    e.message.store(message, Ordering::Release);
    Some(e)
}
