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

//! Where a NONOS disk keeps its store, plan and data, by partition type.

use super::entry::Entry;

/// The NONOS partition types, as `userland/nonos_disk/src/guid/nonos.rs`
/// writes them; kernel_proofs holds the two equal.
pub(crate) const NONOS_STORE: [u8; 16] = [
    0x1A, 0x36, 0xFB, 0x94, 0x00, 0xD5, 0x98, 0x47, 0x93, 0x6B, 0x06, 0xEB, 0x46, 0x85, 0xAA, 0x8F,
];
pub(crate) const NONOS_PLAN: [u8; 16] = [
    0xD3, 0xE6, 0x5A, 0x6F, 0xE5, 0x8F, 0x36, 0x48, 0xBA, 0x18, 0x81, 0x9E, 0x9F, 0xAC, 0xC9, 0x00,
];
pub(crate) const NONOS_DATA: [u8; 16] = [
    0x9E, 0x30, 0xB2, 0x2F, 0x9C, 0x8C, 0xE4, 0x4D, 0xA9, 0x41, 0x01, 0x61, 0xEB, 0x22, 0x2B, 0x66,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Extent {
    pub first: u64,
    pub sectors: u64,
}

impl Extent {
    pub fn end(&self) -> u64 {
        self.first + self.sectors
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Places {
    pub store: Extent,
    /// The plan's sector, then the key header's.
    pub plan: Extent,
    pub data: Option<Extent>,
}

/// The one partition of `type_guid`, `None` when there is none or more than
/// one: two stores would leave the kernel to guess which one vfs means.
fn only(entries: &[Entry], type_guid: [u8; 16]) -> Option<Extent> {
    let mut found = entries.iter().filter(|e| e.type_guid == type_guid);
    let e = found.next()?;
    let sectors = e.last - e.first + 1;
    found.next().is_none().then_some(Extent { first: e.first, sectors })
}

/// A store and a plan with room for the key header behind it; the data
/// partition when the table has one.
pub(crate) fn places(entries: &[Entry]) -> Option<Places> {
    let store = only(entries, NONOS_STORE)?;
    let plan = only(entries, NONOS_PLAN).filter(|p| p.sectors >= 2)?;
    let data = only(entries, NONOS_DATA);
    Some(Places { store, plan, data })
}

/// Whether the table names the partition the loader was read from.
pub(crate) fn holds(entries: &[Entry], unique: &[u8; 16]) -> bool {
    entries.iter().any(|e| &e.unique == unique)
}
