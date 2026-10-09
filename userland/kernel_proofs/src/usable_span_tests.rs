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

/*
 * The frame allocator's total is the RAM the boot map calls usable, not the
 * span it manages. On a q35 guest with 4 GiB the span holds a 2 GiB PCI
 * hole, and the setup wizard read 6.4 GB. These run the kernel's span walk
 * through `#[path]`.
 */

use crate::memory::phys::usable_span::walk_usable;

const GIB: u64 = 1 << 30;

fn walk(map: &[(u64, u64)], start: u64, end: u64) -> (u64, Vec<(u64, u64)>) {
    let mut gaps = Vec::new();
    let bytes = walk_usable(map, start, end, |a, b| gaps.push((a, b)));
    (bytes, gaps)
}

/* RAM below 2 GiB less the firmware's top, RAM from 4 to 6 GiB, the hole between. */
#[test]
fn a_hole_in_the_span_is_not_memory() {
    let map = [(0, 0x9f000), (0x100000, 0x7f00_0000), (4 * GIB, 6 * GIB)];
    let (bytes, gaps) = walk(&map, 0x100000, 6 * GIB);
    assert_eq!(bytes, 0x7f00_0000 - 0x100000 + 2 * GIB);
    assert_eq!(gaps, [(0x7f00_0000, 4 * GIB)]);
}

#[test]
fn overlapping_and_touching_regions_count_once() {
    let map = [(0x100000, 0x200000), (0x180000, 0x300000), (0x300000, 0x400000)];
    assert_eq!(walk(&map, 0x100000, 0x400000), (0x300000, vec![]));
}

#[test]
fn what_lies_outside_the_span_is_not_counted() {
    let map = [(0x1000, 0x180000), (0x1ff800, 0x800000)];
    let (bytes, gaps) = walk(&map, 0x100000, 0x200000);
    assert_eq!(bytes, 0x80000);
    assert_eq!(gaps, [(0x180000, 0x1ff800)]);
}

#[test]
fn no_usable_region_leaves_the_whole_span_a_gap() {
    assert_eq!(walk(&[], 0x100000, 0x200000), (0, vec![(0x100000, 0x200000)]));
}
