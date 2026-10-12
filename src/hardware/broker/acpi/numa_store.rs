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

//! A device's proximity domain: the SRAT Generic Initiator that names it
//! (by PCI function, or by `_HID` and `_UID` for a platform device), else
//! for a PCI function the _PXM of the root bridge whose buses hold it, as
//! the ACPI capsule published it. Bus ranges are replaced when published
//! again, so a restarted capsule answers as the first did.

use super::super::pci_index;
use super::kernel::platform::fw;
use super::numa_node::{bus_domain, BusRange, Region};
use super::platform_paths::named;
use alloc::vec::Vec;
use fw::parser::other::srat_entry::Handle;
use spin::Mutex;

/// More root bridges than any machine has.
const MAX_BRIDGES: usize = 64;

static BUSES: Mutex<Vec<BusRange>> = Mutex::new(Vec::new());

/// False when the store is full.
pub fn publish(r: BusRange) -> bool {
    let mut b = BUSES.lock();
    b.retain(|x| (x.segment, x.first) != (r.segment, r.first));
    (b.len() < MAX_BRIDGES).then(|| b.push(r)).is_some()
}

pub fn device_domain(device_id: u64) -> Option<u32> {
    let initiators = fw::parser::other::numa_initiators();
    if let Some(h) = pci_index::lookup(device_id) {
        let a = h.address;
        let devfn = a.device << 3 | a.function;
        let pci = Handle::Pci { segment: a.segment, bus: a.bus, devfn };
        let named_here = initiators.iter().find(|(_, h)| *h == pci).map(|(d, _)| *d);
        return named_here.or_else(|| bus_domain(&BUSES.lock(), a.segment, a.bus));
    }
    let (hid, uid, _) = named(device_id)?;
    let ours = |h: &Handle| matches!(h, Handle::Acpi { hid: h, uid: u } if *h == hid && u64::from(*u) == uid);
    initiators.iter().find(|(_, h)| ours(h)).map(|(d, _)| *d)
}

/// SRAT's enabled memory ranges by domain.
pub fn regions() -> Vec<Region> {
    let r = fw::numa_regions().into_iter();
    r.map(|m| Region { base: m.base, len: m.length, domain: m.proximity_domain }).collect()
}

pub fn distance(from: u32, to: u32) -> Option<u8> {
    fw::parser::other::numa_distance(from, to)
}
