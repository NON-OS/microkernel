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

//! The broker's device calls on AMD-Vi: a device table entry per requester
//! id, and coverage from the IVHD device entries.

use crate::arch::x86_64::iommu::amd_vi::domain::{attach, detach, device_id};
use crate::arch::x86_64::acpi::parser::other::amd_iommu_aliased;
use crate::arch::x86_64::iommu::amd_vi::unit_covers;
use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

use super::requester::segment0;

fn id(device: DeviceAddress) -> Result<u16, IommuError> {
    let (bus, dev, function) = segment0(device)?;
    Ok(device_id(bus, dev, function))
}

pub(super) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    attach(domain.as_u16(), id(device)?).map_err(|_| IommuError::DeviceAttachFailed)
}

pub(super) fn detach_device(_domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    detach(id(device)?).map_err(|_| IommuError::DeviceDetachFailed)
}

/*
 * An id that shares its entry with another, either way round, is never
 * counted as confined: its DMA would walk the other's tables, or a
 * passthrough entry, while the broker reported it held to its own (L50).
 */
pub(super) fn translates(device: DeviceAddress) -> bool {
    segment0(device).is_ok_and(|(bus, dev, function)| {
        unit_covers(bus, dev, function) && !amd_iommu_aliased(device_id(bus, dev, function))
    })
}
