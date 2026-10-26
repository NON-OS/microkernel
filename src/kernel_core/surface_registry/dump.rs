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

use super::table::SLOTS;

pub fn dump_surface_accounting() {
    let slots = SLOTS.lock();
    let mut count = 0u64;
    let mut frames = 0u64;
    crate::sys::serial::println(b"[OOM] surfaces");
    for slot in slots.iter().flatten() {
        count = count.saturating_add(1);
        frames = frames.saturating_add(slot.frames.len() as u64);
        let mut l = crate::sys::serial::Line::new();
        l.str(b"[OOM] surface owner_pid=").hex(slot.owner_pid as u64).str(b" name=");
        match crate::process::core::PROCESS_TABLE.find_by_pid(slot.owner_pid) {
            Some(pcb) => l.str(pcb.name.lock().as_bytes()),
            None => l.str(b"unknown"),
        };
        l.str(b" bytes=").hex(slot.byte_len).str(b" refs=").hex(slot.refcount as u64).end();
    }
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[OOM] surface_count=").hex(count).str(b" surface_frames=").hex(frames).end();
    let (allocated, regions) = crate::memory::frame_alloc::get_stats();
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[OOM] frame_allocated=").hex(allocated as u64).str(b" frame_regions=");
    l.hex(regions as u64).end();
    let heap = crate::memory::heap::get_heap_stats();
    let mut l = crate::sys::serial::Line::new();
    l.str(b"[OOM] heap_current=").hex(heap.current_usage as u64).str(b" heap_peak=");
    l.hex(heap.peak_usage as u64).end();
}
