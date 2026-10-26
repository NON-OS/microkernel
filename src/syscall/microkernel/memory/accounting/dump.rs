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

use super::state::{EVENTS, EVENT_CAP, EVENT_CURSOR, OWNERS, SYSTEM_BYTES};

pub fn dump_mmap_accounting() {
    crate::sys::serial::println(b"[OOM] mmap owners");
    for owner in OWNERS.lock().iter().copied().filter(|o| o.pid != 0 && o.bytes != 0) {
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[OOM] mmap pid=").hex(owner.pid as u64).str(b" bytes=").hex(owner.bytes).end();
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[OOM] mmap total_system=").hex(SYSTEM_BYTES.load(Ordering::Relaxed)).end();
    let events = EVENTS.lock();
    let start = EVENT_CURSOR.load(Ordering::Relaxed).saturating_sub(EVENT_CAP);
    for n in 0..EVENT_CAP {
        let event = events[(start + n) % EVENT_CAP];
        if event.pid == 0 {
            continue;
        }
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[OOM] mmap_event kind=").hex(event.kind as u64).str(b" pid=");
        l.hex(event.pid as u64).str(b" size=").hex(event.size).str(b" va=").hex(event.va);
        l.str(b" owner=").hex(event.owner).str(b" system=").hex(event.system).end();
    }
}
