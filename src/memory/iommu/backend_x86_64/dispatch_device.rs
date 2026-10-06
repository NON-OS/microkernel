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

//! Device calls, routed by the vendor selected at boot.

use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

use super::route::{route, Backend};

pub(crate) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    match route(b"attach_device")? {
        Backend::Vtd => super::device::attach_device(domain, device),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_device::attach_device(domain, device),
    }
}

pub(crate) fn detach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    match route(b"detach_device")? {
        Backend::Vtd => super::device::detach_device(domain, device),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_device::detach_device(domain, device),
    }
}

pub(crate) fn translates(device: DeviceAddress) -> bool {
    match route(b"translates") {
        Ok(Backend::Vtd) => super::device::translates(device),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Ok(Backend::AmdVi) => super::amd_device::translates(device),
        Err(_) => false,
    }
}
