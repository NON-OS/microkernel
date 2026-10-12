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

//! The PCIe ports firmware calls external-facing and the buses behind each,
//! with DEVICE_FLAG_EXTERNAL set on every function they reach, those added
//! after the boot scan included. Kept beside the table rather than in the
//! ACPI broker: the table and the IOMMU that reads the flag are not x86's.
//! A flag once set stays set: a port is never less external than it was.

extern crate alloc;

use alloc::vec::Vec;
use spin::Mutex;

use super::state::TABLE;
use crate::drivers::pci::types::PciAddress;
use crate::hardware::broker::external_range::{behind, ExternalRange};
use crate::hardware::broker::{pci_index, DEVICE_FLAG_EXTERNAL};

// Taken before TABLE and never while holding it.
static RANGES: Mutex<Vec<ExternalRange>> = Mutex::new(Vec::new());

fn bdf(a: &PciAddress) -> (u8, u8, u8) {
    (a.bus, a.device, a.function)
}

/// The range a function sits in, if any: for the IOMMU's domain choice.
#[allow(dead_code)] // the IOMMU half reads it; x86 alone declares ports yet
pub fn external_port_of(a: &PciAddress) -> Option<ExternalRange> {
    RANGES.lock().iter().find(|r| behind(r, a.segment, bdf(a))).copied()
}

/// Records the range (again, for a capsule that restarted) and flags each
/// function in it, answering how many the table holds.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
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
pub(super) fn flag_for(a: &PciAddress) -> u32 {
    let behind_any = RANGES.lock().iter().any(|r| behind(r, a.segment, bdf(a)));
    if behind_any {
        DEVICE_FLAG_EXTERNAL
    } else {
        0
    }
}
