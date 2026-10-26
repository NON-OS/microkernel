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

//! The ACPI broker's writes to the external ranges: a port marked, and the flag
//! a function found after boot starts with.

use alloc::vec::Vec;

use super::super::state::TABLE;
use super::{bdf, RANGES};
use crate::drivers::pci::types::PciAddress;
use crate::hardware::broker::external_range::{behind, ExternalRange};
use crate::hardware::broker::{pci_index, DEVICE_FLAG_EXTERNAL};

/// Records the range (again, for a capsule that restarted) and flags each
/// function in it, answering how many the table holds.
pub(in crate::hardware::broker) fn mark(range: ExternalRange) -> usize {
    {
        let mut ranges = RANGES.lock();
        ranges.retain(|r| (r.segment, r.port) != (range.segment, range.port));
        ranges.push(range);
    }
    let ids: Vec<u64> = pci_index::all()
        .into_iter()
        .filter(|h| behind(&range, h.address.segment, bdf(&h.address)))
        .map(|h| h.device_id)
        .collect();
    let mut table = TABLE.write();
    for r in table.iter_mut().filter(|r| ids.contains(&r.device_id)) {
        r.flags |= DEVICE_FLAG_EXTERNAL;
    }
    ids.len()
}

/// The flag a function found after boot starts with.
pub(in crate::hardware::broker::table) fn flag_for(a: &PciAddress) -> u32 {
    let behind_any = RANGES.lock().iter().any(|r| behind(r, a.segment, bdf(a)));
    if behind_any {
        DEVICE_FLAG_EXTERNAL
    } else {
        0
    }
}
