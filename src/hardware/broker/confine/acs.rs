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

//! A device's isolation group, read from the hardware: the functions
//! the broker table enumerated, each bridge's buses, and each hop's ACS,
//! turned on where it is offered and read back (acs_read.rs).

extern crate alloc;

use alloc::vec::Vec;

use super::acs_kind::conventional;
use super::acs_path::path;
use super::acs_read::{buses, redirect};
use super::acs_rule::group as reached;
use super::acs_tree::{Bdf, Func};
use crate::hardware::broker::pci_index::{self, PciHandle};

/// The other broker devices `device_id` reaches without the IOMMU; None for
/// one behind a VMD, which never confines, or whose path does not end.
pub(super) fn group(device_id: u64) -> Option<Vec<u64>> {
    let me = pci_index::lookup(device_id).filter(|h| h.address.segment == 0)?;
    let handles: Vec<PciHandle> =
        pci_index::all().into_iter().filter(|h| h.address.segment == 0).collect();
    let all: Vec<Func> = handles.iter().map(|h| func(bdf(h))).collect();
    let hops = path(bdf(&me), &all)?;
    let reached = reached(hops, redirect);
    let others = handles.iter().filter(|h| h.device_id != device_id);
    Some(others.filter(|h| reached.contains(&bdf(h))).map(|h| h.device_id).collect())
}

fn func(at: Bdf) -> Func {
    let buses = buses(at);
    Func { at, buses, conventional: buses.is_some() && conventional(at) }
}

fn bdf(h: &PciHandle) -> Bdf {
    (h.address.bus, h.address.device, h.address.function)
}
