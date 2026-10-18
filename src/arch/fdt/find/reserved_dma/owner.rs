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

//! Whose an `iommu-addresses` entry is, as the remapping unit sees it.

use crate::arch::fdt::endian::be_u32;
use crate::arch::fdt::error::FdtError;
use crate::arch::fdt::find::pci::bus_range;
use crate::arch::fdt::node::NodeRef;
use crate::arch::fdt::parser::Fdt;

use super::entry::Owner;
use super::streams::streams;

/// The owner, or why the entry names no device the kernel can key it by.
pub(super) type Found = Result<Owner, &'static [u8]>;

fn is_pci_bridge(fdt: &Fdt, node: &NodeRef) -> Result<bool, FdtError> {
    Ok(fdt.property(node, b"device_type")?.is_some_and(|t| t.string() == Ok(b"pci")))
}

/// Segment 0 is the only one this kernel's requester ids name.
fn other_domain(fdt: &Fdt, bridge: &NodeRef) -> Result<bool, FdtError> {
    Ok(fdt.property(bridge, b"linux,pci-domain")?.is_some_and(|d| d.u32() != Ok(0)))
}

pub(super) fn owner(fdt: &Fdt, device: &NodeRef) -> Result<Found, FdtError> {
    if is_pci_bridge(fdt, device)? {
        if other_domain(fdt, device)? {
            return Ok(Err(b"a PCI domain other than 0"));
        }
        let (first_bus, last_bus) = bus_range(fdt, device)?;
        return Ok(Ok(Owner::PciBridge { first_bus, last_bus }));
    }
    let chain = fdt.chain(device)?;
    let parent = chain.as_slice().iter().rev().nth(1).copied();
    if let Some(bridge) = parent.filter(|p| is_pci_bridge(fdt, p).unwrap_or(false)) {
        if other_domain(fdt, &bridge)? {
            return Ok(Err(b"a PCI domain other than 0"));
        }
        // phys.hi of the function's `reg`: bus 23:16, device 15:11,
        // function 10:8 (PCI bus binding, recalled) is its requester id.
        let Some(reg) = fdt.property(device, b"reg")? else {
            return Ok(Err(b"a PCI function with no reg"));
        };
        return Ok(Ok(Owner::Pci { rid: (be_u32(reg.data, 0)? >> 8) as u16 }));
    }
    streams(fdt, device)
}
