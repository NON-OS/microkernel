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

//! A device's isolation group and requester-id aliases, read from the
//! hardware (K11): the functions the broker table enumerated, each bridge's
//! buses and kind, and each hop's ACS, turned on where it is offered and
//! read back (acs_read.rs).

extern crate alloc;

use alloc::vec::Vec;

use super::acs_alias::aliases;
use super::acs_kind::{secondary, Secondary};
use super::acs_path::path;
use super::acs_read::{buses, redirect};
use super::acs_rule::group as reached;
use super::acs_tree::{Bdf, Func};
use crate::hardware::broker::pci_index::{self, PciHandle};

/// What a device reaches past the IOMMU, and the ids it reaches it under.
pub(super) struct Reach {
    /// The other broker devices in its isolation group.
    pub group: Vec<u64>,
    /// The requester ids its DMA may carry besides its own.
    pub aliases: Vec<Bdf>,
}

/// None for a device behind a VMD, which never confines, or whose path to
/// the root does not end.
pub(super) fn reach(device_id: u64) -> Option<Reach> {
    let me = pci_index::lookup(device_id).filter(|h| h.address.segment == 0)?;
    let handles: Vec<PciHandle> =
        pci_index::all().into_iter().filter(|h| h.address.segment == 0).collect();
    let all: Vec<Func> = handles.iter().map(|h| func(bdf(h))).collect();
    let hops = path(bdf(&me), &all)?;
    let reached = reached(hops, redirect);
    let others = handles.iter().filter(|h| h.device_id != device_id);
    let group = others.filter(|h| reached.contains(&bdf(h))).map(|h| h.device_id).collect();
    Some(Reach { group, aliases: aliases(bdf(&me), &all) })
}

fn func(at: Bdf) -> Func {
    let buses = buses(at);
    let kind = buses.map(|_| secondary(at));
    let alias = match (kind, buses) {
        (Some(Secondary::PcieToPci), Some((sec, _))) => Some((sec, 0, 0)),
        (Some(Secondary::LegacyPci), _) => Some(at),
        _ => None,
    };
    let conventional = kind.is_some_and(|k| k != Secondary::Express);
    Func { at, buses, conventional, alias }
}

fn bdf(h: &PciHandle) -> Bdf {
    (h.address.bus, h.address.device, h.address.function)
}
