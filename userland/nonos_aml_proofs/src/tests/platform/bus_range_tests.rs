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

//! A root bridge's bus numbers from its _CRS: the first bus-number
//! descriptor of any width; memory and I/O descriptors before it are
//! passed over; none, an empty one, or one past bus 255 gives nothing.

use crate::platform::bus_range;

fn word(kind: u8, min: u16, len: u16) -> Vec<u8> {
    let max = min.wrapping_add(len).wrapping_sub(1);
    let f = [0, min, max, 0, len].map(u16::to_le_bytes).concat();
    [vec![0x88, 13, 0, kind, 0, 0], f].concat()
}

fn qword(kind: u8, min: u64, len: u64) -> Vec<u8> {
    let f = [0, min, min + len - 1, 0, len].map(u64::to_le_bytes).concat();
    [vec![0x8A, 43, 0, kind, 0, 0], f].concat()
}

fn crs(parts: &[Vec<u8>]) -> Vec<u8> {
    [parts.concat(), vec![0x79, 0]].concat()
}

#[test]
fn bus_numbers_of_a_root_bridge() {
    assert_eq!(bus_range(&crs(&[word(2, 0, 0x80)])), Some((0, 0x7F)));
    let mixed = crs(&[qword(0, 0xE000_0000, 0x1000), word(1, 0xCF8, 8), word(2, 0x80, 0x80)]);
    assert_eq!(bus_range(&mixed), Some((0x80, 0xFF)), "memory and I/O passed over");
    assert_eq!(bus_range(&crs(&[qword(2, 0x40, 0x10)])), Some((0x40, 0x4F)));
    assert_eq!(bus_range(&crs(&[word(1, 0xCF8, 8)])), None);
    assert_eq!(bus_range(&crs(&[word(2, 0, 0)])), None, "no bus");
    assert_eq!(bus_range(&crs(&[word(2, 0x80, 0x100)])), None, "past bus 255");
    assert_eq!(bus_range(&[]), None);
}
