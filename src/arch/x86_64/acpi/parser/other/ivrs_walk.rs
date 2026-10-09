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

//! The AMD IOMMU register bases an IVRS table describes. Pure, over the
//! table's bytes, so the host proofs can feed it hostile tables.

/// IVRS: a 36-byte SDT header, IVinfo, eight reserved bytes, then IVDB blocks.
const FIRST_BLOCK: usize = 48;

/// IVHD block types: 10h (legacy), 11h and 40h (extended). One IOMMU is
/// usually described by a 10h block and again by an 11h or 40h one.
const IVHD_TYPES: [u8; 3] = [0x10, 0x11, 0x40];

pub const MAX_AMD_IOMMUS: usize = 8;

/// Distinct IOMMU base addresses, in table order, written to `out`; returns
/// how many. Stops at the first block whose length is short or runs past the
/// table, or when `out` is full; never reads out of bounds.
pub fn iommu_bases(table: &[u8], out: &mut [u64]) -> usize {
    let mut found = 0;
    let mut at = FIRST_BLOCK;
    while at + 4 <= table.len() {
        let kind = table[at];
        let length = u16::from_le_bytes([table[at + 2], table[at + 3]]) as usize;
        if length < 4 || at + length > table.len() {
            break;
        }
        if IVHD_TYPES.contains(&kind) && length >= 24 {
            let mut raw = [0u8; 8];
            raw.copy_from_slice(&table[at + 8..at + 16]);
            let base = u64::from_le_bytes(raw);
            if base != 0 && !out[..found].contains(&base) {
                if found == out.len() {
                    break;
                }
                out[found] = base;
                found += 1;
            }
        }
        at += length;
    }
    found
}
