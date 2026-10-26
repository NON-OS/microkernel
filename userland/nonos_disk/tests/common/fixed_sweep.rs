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

//! Seeded tables for the choice of the fixed sectors, and the rule stated
//! apart from the kernel: a table naming NONOS's store and plan places them;
//! otherwise the fixed store (256 up to 245760) and plan (up to 262144) are
//! used, unless a partition of any type but NONOS's own crosses them.

use crate::kernel_gpt::places::{NONOS_DATA, NONOS_PLAN, NONOS_STORE};
use crate::kernel_gpt::{layout, Entry, Held, Layout, Table};

/// The EFI System Partition's type, as its GUID is laid out on disk
/// (UEFI 2.10 table 5.7, as recalled, unverified).
pub const ESP: [u8; 16] = [
    0x28, 0x73, 0x2A, 0xC1, 0x1F, 0xF8, 0xD2, 0x11, 0xBA, 0x4B, 0x00, 0xA0, 0xC9, 0x3E, 0xC9, 0x3B,
];
pub const FIXED_LO: u64 = 256;
pub const FIXED_HI: u64 = 262_144;
/// Each fixed range's first and last sector, and the sectors just outside.
pub const EDGES: [u64; 6] = [FIXED_LO - 1, FIXED_LO, 245_759, 245_760, FIXED_HI - 1, FIXED_HI];

pub fn entry(type_guid: [u8; 16], first: u64, last: u64) -> Entry {
    Entry { type_guid, unique: [7; 16], first, last }
}

/// What the rule places: `Some((store, plan, named))` by first LBA, or
/// `None` when the fixed sectors are refused.
pub fn rule(entries: &[Entry]) -> Option<(u64, u64, bool)> {
    let one = |t: [u8; 16]| {
        let of: Vec<&Entry> = entries.iter().filter(|e| e.type_guid == t).collect();
        (of.len() == 1).then(|| of[0])
    };
    match (one(NONOS_STORE), one(NONOS_PLAN).filter(|p| p.last - p.first >= 1)) {
        (Some(s), Some(p)) => Some((s.first, p.first, true)),
        _ => {
            let ours = |e: &Entry| [NONOS_STORE, NONOS_PLAN, NONOS_DATA].contains(&e.type_guid);
            let crosses =
                entries.iter().any(|e| !ours(e) && e.first < FIXED_HI && e.last >= FIXED_LO);
            (!crosses).then_some((FIXED_LO, 245_760, false))
        }
    }
}

pub fn kernel(entries: Vec<Entry>) -> Option<(u64, u64, bool)> {
    match layout(Table::Named(entries.clone(), Held::Primary)) {
        Layout::At { places, entries: kept, named, .. } => {
            assert_eq!(kept, entries, "every partition named, kept for the loader's");
            Some((places.store.first, places.plan.first, named))
        }
        Layout::Crossed => None,
    }
}
