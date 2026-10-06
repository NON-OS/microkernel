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

//! Which requester ids the AMD IOMMUs in an IVRS table cover, from the IVHD
//! device entries (AMD IOMMU spec 48882, 5.2.2.2), read as Linux reads them
//! in amd/init.c, init_iommu_from_acpi. Pure, over the table's bytes.

use super::ivhd_entry::entries;

pub const MAX_SPANS: usize = 64;

/// Requester ids `first..=last`. `named` marks an id firmware listed by
/// itself (select, alias, special, ACPI HID): a device that may not appear
/// in PCI enumeration but still issues DMA under that id.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Span {
    pub first: u16,
    pub last: u16,
    pub named: bool,
}

/// IVRS: a 36-byte SDT header, IVinfo, eight reserved bytes, then blocks.
const FIRST_BLOCK: usize = 48;

/// Spans in table order, written to `out`; returns how many. Stops at the
/// first malformed block or entry, or when `out` is full. IVHD 10h has a
/// 24-byte header, 11h and 40h a 40-byte one; other blocks are skipped.
pub fn spans(table: &[u8], out: &mut [Span]) -> usize {
    let mut n = 0;
    let mut at = FIRST_BLOCK;
    while at + 4 <= table.len() {
        let kind = table[at];
        let length = u16::from_le_bytes([table[at + 2], table[at + 3]]) as usize;
        if length < 4 || at + length > table.len() {
            break;
        }
        let header = match kind {
            0x10 => 24,
            0x11 | 0x40 => 40,
            _ => length,
        };
        if header < length {
            n = entries(&table[at + header..at + length], out, n);
        }
        at += length;
    }
    n
}

/// Whether any span covers requester id `id`.
pub fn covers(spans: &[Span], id: u16) -> bool {
    spans.iter().any(|s| s.first <= id && id <= s.last)
}
