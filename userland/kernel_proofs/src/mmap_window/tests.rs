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

use super::map_window::{map_window, MAX_MAP_ENTRIES};

const BASE: u64 = 0xFFFF_8000_0000_0000;
const SIZE: u64 = 0x0000_0040_0000_0000;
const ENTRY: u32 = 40;

#[test]
fn a_map_in_ram_is_read_at_its_direct_map_address() {
    assert_eq!(map_window(0x7F00_0000, ENTRY, 120, BASE, SIZE), Some((BASE + 0x7F00_0000, 120)));
    let last = SIZE - 40 * 3;
    assert_eq!(map_window(last, ENTRY, 3, BASE, SIZE), Some((BASE + last, 3)), "ends at the top");
}

#[test]
fn a_map_the_direct_map_does_not_hold_whole_is_none() {
    assert_eq!(map_window(SIZE - 40, ENTRY, 2, BASE, SIZE), None, "runs one entry past");
    assert_eq!(map_window(SIZE, ENTRY, 1, BASE, SIZE), None, "starts past");
    assert_eq!(map_window(u64::MAX - 7, ENTRY, 1, BASE, SIZE), None, "an end that overflows");
}

#[test]
fn an_empty_misaligned_or_overlong_map_is_none() {
    assert_eq!(map_window(0, ENTRY, 10, BASE, SIZE), None);
    assert_eq!(map_window(0x1000, ENTRY, 0, BASE, SIZE), None);
    assert_eq!(map_window(0x1004, ENTRY, 10, BASE, SIZE), None);
    assert!(map_window(0x1000, ENTRY, MAX_MAP_ENTRIES, BASE, SIZE).is_some());
    assert_eq!(map_window(0x1000, ENTRY, MAX_MAP_ENTRIES + 1, BASE, SIZE), None);
    assert_eq!(map_window(0x1000, ENTRY, u32::MAX, BASE, SIZE), None);
}

#[test]
fn no_window_reaches_outside_the_direct_map() {
    let mut x = 0x9E37_79B9_7F4A_7C15u64;
    for _ in 0..200_000 {
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        let ptr = (x >> (x & 31)) & !7;
        let count = (x >> 40) as u32 & 0x3_FFFF;
        if let Some((virt, n)) = map_window(ptr, ENTRY, count, BASE, SIZE) {
            assert_eq!(n, count as usize);
            assert!(virt >= BASE && virt - BASE + n as u64 * 40 <= SIZE, "{ptr:#x}+{count}");
        }
    }
}
