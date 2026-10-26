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
//! IVHD device entries for the IVRS sweep (AMD IOMMU specification 48882,
//! section 5.2.2.2): four bytes below 40h, eight from 40h, and the variable
//! ACPI one (F0h), whose UID length sometimes lies.

use super::gen::Rng;

/// Device entries by kind, with their sizes: pad, all, select, range start
/// and end; alias select and start, extended select and start, special;
/// and the variable ACPI one, its size made below.
const ENTRIES: [(u8, usize); 11] = [
    (0, 4),
    (1, 4),
    (2, 4),
    (3, 4),
    (4, 4),
    (0x42, 8),
    (0x43, 8),
    (0x46, 8),
    (0x47, 8),
    (0x48, 8),
    (0xF0, 0),
];

/// One device entry, one time in ten of a kind no IVHD defines.
pub fn entry(r: &mut Rng) -> Vec<u8> {
    let (kind, size) = if r.one_in(10) { (r.next() as u8, 4) } else { r.pick(&ENTRIES) };
    let mut e = [vec![kind], r.bytes(size.max(4) - 1)].concat();
    if kind == 0xF0 {
        // HID, CID, UID format, then a UID length byte that sometimes lies.
        let uid = r.below(12);
        e.extend(r.bytes(17).into_iter().chain([uid as u8]).chain(r.bytes(uid)));
        e[21] = if r.one_in(6) { r.next() as u8 } else { uid as u8 };
    }
    e
}
