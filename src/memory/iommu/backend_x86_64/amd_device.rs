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

use crate::arch::x86_64::amd_vi::domain::{attach, detach, device_id};
use crate::arch::x86_64::amd_vi::unit_covers;
use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

fn id(device: DeviceAddress) -> u16 {
    device_id(device.pci_bus(), device.pci_device(), device.pci_function())
}

pub(super) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    attach(domain.as_u16(), id(device)).map_err(|_| IommuError::DeviceAttachFailed)
}

pub(super) fn detach_device(_domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    detach(id(device)).map_err(|_| IommuError::DeviceDetachFailed)
}

pub(super) fn translates(device: DeviceAddress) -> bool {
    unit_covers(device.pci_bus(), device.pci_device(), device.pci_function())
}
