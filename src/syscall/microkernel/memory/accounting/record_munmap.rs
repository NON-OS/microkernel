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

pub(crate) fn record_munmap(pid: u32, size: usize, va: u64) {
    let size = size as u64;
    let old = SYSTEM_BYTES
        .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |v| Some(v.saturating_sub(size)))
        .unwrap_or_else(|v| v);
    let system = old.saturating_sub(size);
    let mut total = 0;
    {
        let mut owners = OWNERS.lock();
        if let Some(owner) = owners.iter_mut().find(|o| o.pid == pid) {
            owner.bytes = owner.bytes.saturating_sub(size);
            total = owner.bytes;
        }
    }
    let idx = EVENT_CURSOR.fetch_add(1, Ordering::Relaxed) % EVENT_CAP;
    EVENTS.lock()[idx] = Event { pid, kind: b'U', size, va, owner: total, system };
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[MUNMAP] pid=").hex(pid as u64).str(b" name=");
    match crate::process::core::PROCESS_TABLE.find_by_pid(pid) {
        Some(pcb) => l.who(pcb.name.lock().as_bytes()),
        None => l.str(b"unknown"),
    };
    l.str(b" size=").hex(size).str(b" va=").hex(va).str(b" total_pid=").hex(total);
    l.str(b" total_system=").hex(system).end();
}
