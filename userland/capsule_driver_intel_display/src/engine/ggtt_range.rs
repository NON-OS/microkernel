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

//! Where in the GGTT the driver's surfaces go. The table's size is in
//! the GMCH Graphics Control register (PCI config 0x50): GGMS in bits 7:6
//! from Broadwell, 0 none, else 2^n MiB of 8 byte entries; on Haswell in
//! bits 9:8, 0 none, 1 one MiB, 2 two MiB, of 4 byte entries (field
//! positions unverified). Firmware's own scanout surface sits low in the
//! table, so the driver takes runs from the top, clear of it.

use super::ggtt_pte::Pte;

/// Entries in the GGTT, each mapping 4 KiB.
pub fn entries(t: Pte, ggc: u16) -> u32 {
    let mib = match (t, (ggc >> 6) & 3, (ggc >> 8) & 3) {
        (Pte::Wide, 0, _) | (Pte::Narrow, _, 0 | 3) => 0,
        (Pte::Wide, n, _) => 1u32 << n,
        (Pte::Narrow, _, n) => n as u32,
    };
    mib * (1024 * 1024 / t.size())
}

/// A run of `pages` entries, aligned to `align` entries, as high as it
/// goes below `total`, not touching the entries `avoid.0..avoid.1`. The
/// first entry, or None when no such run exists.
pub fn pick(total: u32, pages: u32, align: u32, avoid: (u32, u32)) -> Option<u32> {
    if pages == 0 || align == 0 || pages > total {
        return None;
    }
    let mut start = (total - pages) / align * align;
    loop {
        let end = start + pages;
        if end <= avoid.0 || start >= avoid.1 {
            return Some(start);
        }
        // Jump below the firmware's surface and try once more there.
        let below = avoid.0.checked_sub(pages)? / align * align;
        if below >= start {
            return None;
        }
        start = below;
    }
}
