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

//! The partitions an entry array names, each held inside the header's
//! usable range, the array's CRC checked first.

use alloc::vec::Vec;

use super::bytes::le64;
use super::crc32::crc32;
use super::header::{Header, ENTRY_SIZE};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Entry {
    pub(crate) type_guid: [u8; 16],
    pub(crate) unique: [u8; 16],
    pub(crate) first: u64,
    pub(crate) last: u64,
}

/// The used entries of `array`, the whole array `h` names. `None` when its
/// CRC fails: a damaged table names nothing.
pub(crate) fn entries(h: &Header, array: &[u8]) -> Option<Vec<Entry>> {
    let len = h.entries as usize * ENTRY_SIZE;
    let array = array.get(..len)?;
    if crc32(array) != h.entries_crc {
        return None;
    }
    let mut out = Vec::new();
    for e in array.chunks_exact(ENTRY_SIZE) {
        let mut type_guid = [0u8; 16];
        type_guid.copy_from_slice(&e[..16]);
        if type_guid == [0; 16] {
            continue;
        }
        let mut unique = [0u8; 16];
        unique.copy_from_slice(&e[16..32]);
        let (first, last) = (le64(e, 32), le64(e, 40));
        // A partition outside the usable range (UEFI 2.10 5.3.3 as recalled,
        // unverified) is not
        // one to read or write through.
        if first < h.first_usable || first > last || last > h.last_usable {
            continue;
        }
        out.push(Entry { type_guid, unique, first, last });
    }
    Some(out)
}
