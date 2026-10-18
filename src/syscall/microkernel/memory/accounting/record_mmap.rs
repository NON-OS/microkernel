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

use super::state::{Event, EVENTS, EVENT_CAP, EVENT_CURSOR, OWNERS, SYSTEM_BYTES};

pub(crate) fn record_mmap(pid: u32, size: usize, va: u64) {
    let size = size as u64;
    let old = SYSTEM_BYTES
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v.saturating_add(size)))
        .unwrap_or_else(|v| v);
    let system = old.saturating_add(size);
    let mut total = size;
    {
        let mut owners = OWNERS.lock();
        if let Some(owner) = owners.iter_mut().find(|o| o.pid == pid) {
            owner.bytes = owner.bytes.saturating_add(size);
            total = owner.bytes;
        } else if let Some(owner) = owners.iter_mut().find(|o| o.pid == 0) {
            owner.pid = pid;
            owner.bytes = size;
        }
    }
    let idx = EVENT_CURSOR.fetch_add(1, Ordering::Relaxed) % EVENT_CAP;
    EVENTS.lock()[idx] = Event { pid, kind: b'M', size, va, owner: total, system };
}
