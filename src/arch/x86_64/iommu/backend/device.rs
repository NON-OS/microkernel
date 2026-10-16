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

//! Binding a PCI function to a domain by writing its context entry.

use crate::arch::x86_64::iommu::device::{map_device, unmap_device};
use crate::arch::x86_64::iommu::domain::DomainId as VtdDomainId;
use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

use super::enforced;
use super::requester::segment0;

/// Gated: a context entry written while translation is off changes nothing
/// about where the device's transactions land, so reporting success would be
/// reporting a confinement that does not exist.
pub(crate) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    enforced::require()?;
    let (bus, dev, function) = segment0(device)?;
    map_device(VtdDomainId::new(domain.as_u16()), bus, dev, function)
        .map_err(|_| IommuError::DeviceAttachFailed)
}

pub(crate) fn detach_device(_domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    let (bus, dev, function) = segment0(device)?;
    unmap_device(bus, dev, function).map_err(|_| IommuError::DeviceDetachFailed)
}

/// Whether a unit this kernel has in service translates `device`: VT-d is
/// enforcing, a programmed unit's scope covers the device, and its DMA
/// arrives under its own id (L51). A device no unit covers would take an IOVA
/// for a physical address.
pub(crate) fn translates(device: DeviceAddress) -> bool {
    let Ok((bus, dev, function)) = segment0(device) else {
        return false;
    };
    enforced::require().is_ok()
        && crate::arch::x86_64::iommu::unit::coverage::unit_covers(bus, dev, function)
        && super::own_id_read::own_id(bus, dev, function)
}
