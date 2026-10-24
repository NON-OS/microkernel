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

//! The partitions an entry array names, the array's CRC checked first
//! (UEFI 2.10, 5.3.3, as recalled, unverified). I/O through a partition
//! must not reach the headers, the entry arrays or another partition, so
//! one outside the usable range, or ending before it starts, is skipped,
//! and a table whose partitions overlap names nothing at all.

use alloc::vec::Vec;

use super::bytes::le64;
use super::crc32::crc32;
use super::header::Header;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) type_guid: [u8; 16],
    pub(crate) unique: [u8; 16],
    pub(crate) first: u64,
    pub(crate) last: u64,
}

/// The used entries of `array`, the whole array `h` names, each read for
/// its first 128 bytes whatever its size. `None` when its CRC fails or two
/// partitions overlap: a damaged table names nothing.
pub(crate) fn entries(h: &Header, array: &[u8]) -> Option<Vec<Entry>> {
    let size = h.entry_size as usize;
    let array = array.get(..h.entries as usize * size)?;
    // A Header comes only from parse, which holds the size to 128 x 2^n.
    if crc32(array) != h.entries_crc {
        return None;
    }
    let mut out = Vec::new();
    for e in array.chunks_exact(size) {
        let mut type_guid = [0u8; 16];
        type_guid.copy_from_slice(&e[..16]);
        if type_guid == [0; 16] {
            continue;
        }
        let mut unique = [0u8; 16];
        unique.copy_from_slice(&e[16..32]);
        let (first, last) = (le64(e, 32), le64(e, 40));
        if first < h.first_usable || first > last || last > h.last_usable {
            continue;
        }
        out.push(Entry { type_guid, unique, first, last });
    }
    (!overlap(&out)).then_some(out)
}

/// Whether any two partitions share a sector.
fn overlap(entries: &[Entry]) -> bool {
    let mut spans: Vec<(u64, u64)> = entries.iter().map(|e| (e.first, e.last)).collect();
    spans.sort_unstable();
    spans.windows(2).any(|w| w[1].0 <= w[0].1)
}
