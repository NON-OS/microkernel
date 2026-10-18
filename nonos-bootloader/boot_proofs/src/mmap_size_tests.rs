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

//! The memory map buffer, sized from the firmware's own map with room for
//! the descriptors added before ExitBootServices, never below the old 1024.

use crate::handoff::jump::mmap_copy::{mmap_capacity, mmap_pages, MMAP_FLOOR, MMAP_SLACK};
use crate::handoff::jump::MemoryMapEntry;

#[test]
fn the_buffer_is_sized_from_the_firmware_map_with_room_to_grow() {
    // OVMF reports 48-byte descriptors; 4000 of them is a large server.
    assert_eq!(mmap_capacity(4000 * 48, 48), 4000 + MMAP_SLACK);
    assert_eq!(mmap_capacity(4000 * 48 + 1, 48), 4001 + MMAP_SLACK, "a part descriptor counts");
    assert_eq!(mmap_capacity(100 * 48, 48), MMAP_FLOOR, "never below the old 1024");
    assert_eq!(mmap_capacity(0, 0), MMAP_FLOOR, "a firmware that says nothing");
    assert_eq!(mmap_capacity(usize::MAX, 1), usize::MAX, "saturates");
}

#[test]
fn the_pages_hold_the_capacity() {
    for cap in [1usize, 102, 103, 1024, 4256, 100_000] {
        let pages = mmap_pages(cap);
        assert!(pages * 4096 >= cap * MemoryMapEntry::SIZE, "{cap} entries fit");
        assert!((pages - 1) * 4096 < cap * MemoryMapEntry::SIZE, "{cap}: no spare page");
    }
    assert_eq!(MemoryMapEntry::SIZE, 40, "the layout the kernel reads");
}
