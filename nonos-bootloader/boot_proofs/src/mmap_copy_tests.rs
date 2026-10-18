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

//! The memory map the loader hands the kernel: every firmware descriptor or
//! none. The copy used to stop at 1024 entries without a word.

use crate::handoff::jump::mmap_copy::{copy_entries, MapOverflow};
use crate::handoff::jump::MemoryMapEntry;

fn entry(n: u64) -> MemoryMapEntry {
    MemoryMapEntry {
        memory_type: 7,
        _pad: 0,
        physical_start: n << 20,
        virtual_start: 0,
        page_count: 256,
        attribute: 0xF,
    }
}

fn blank() -> MemoryMapEntry {
    MemoryMapEntry { memory_type: 0, ..entry(0) }
}

#[test]
fn a_map_that_fits_is_copied_whole_and_in_order() {
    for n in [0usize, 1, 7, 1024, 3000] {
        let mut out = vec![blank(); n + 3];
        assert_eq!(copy_entries((0..n as u64).map(entry), &mut out), Ok(n as u32));
        for (i, e) in out[..n].iter().enumerate() {
            assert_eq!(e.physical_start, (i as u64) << 20, "entry {i}");
        }
        assert!(out[n..].iter().all(|e| e.memory_type == 0), "nothing past the count");
    }
}

#[test]
fn a_map_one_past_the_buffer_is_refused_with_its_whole_count() {
    for room in [0usize, 1, 1024, 1500] {
        let mut out = vec![blank(); room];
        let got = copy_entries((0..room as u64 + 1).map(entry), &mut out);
        assert_eq!(got, Err(MapOverflow { entries: room + 1, room }));
    }
    let mut out = vec![blank(); 10];
    let got = copy_entries((0..5000).map(entry), &mut out);
    assert_eq!(got, Err(MapOverflow { entries: 5000, room: 10 }), "counted to the end");
}
