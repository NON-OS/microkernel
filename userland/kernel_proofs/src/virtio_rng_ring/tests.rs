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

use super::layout::{layout, Layout, MAX_QUEUE, QUEUE_ALIGN};

#[test]
fn a_small_queue_puts_the_used_ring_on_the_next_page() {
    // Eight entries: 128 bytes of descriptors, the available ring after them.
    assert_eq!(layout(8), Some(Layout { avail: 128, used: 4096, bytes: 4096 + 6 + 64 }));
    assert_eq!(layout(16), Some(Layout { avail: 256, used: 4096, bytes: 4096 + 6 + 128 }));
}

#[test]
fn the_largest_queue_fills_two_pages_before_its_used_ring() {
    // 256 descriptors are a page; the available ring spills into the second.
    assert_eq!(layout(256), Some(Layout { avail: 4096, used: 8192, bytes: 8192 + 6 + 2048 }));
}

#[test]
fn every_allowed_size_keeps_the_rings_apart_and_aligned() {
    let mut size = 1u16;
    while size <= MAX_QUEUE {
        let l = layout(size).expect("a power of two up to the maximum");
        let n = size as usize;
        assert_eq!(l.avail, 16 * n, "the available ring follows the descriptors");
        assert!(l.used >= l.avail + 2 * (3 + n), "size {size}: rings overlap");
        assert_eq!(l.used % QUEUE_ALIGN, 0, "size {size}: used ring unaligned");
        assert!(l.bytes <= 3 * 4096, "size {size}: past three pages");
        size *= 2;
    }
}

#[test]
fn sizes_the_spec_does_not_allow_are_refused() {
    for size in [0u16, 3, 6, 12, 100, 255, 512, 1024, u16::MAX] {
        assert_eq!(layout(size), None, "size {size}");
    }
}
