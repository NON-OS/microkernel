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

use super::backend;
use super::device::DeviceAddress;
use super::domain_id::DomainId;
use super::error::IommuError;

/// Owned handle for an IOMMU translation domain. Dropping the handle
/// destroys the domain; its page-table frames are not freed.
pub struct IommuDomain {
    id: DomainId,
}

impl IommuDomain {
    /// Allocate a fresh IOMMU translation domain.
    pub fn allocate() -> Result<Self, IommuError> {
        let id = backend::allocate_domain()?;
        Ok(Self { id })
    }

    /// Identifier of this domain, stable for its lifetime.
    pub const fn id(&self) -> DomainId {
        self.id
    }

    /// Bind a device to this domain so its DMA transactions translate
    /// through this domain's page tables.
    pub fn attach_device(&self, device: DeviceAddress) -> Result<(), IommuError> {
        backend::attach_device(self.id, device)
    }

    /// Reverse a previous `attach_device` for this device.
    pub fn detach_device(&self, device: DeviceAddress) -> Result<(), IommuError> {
        backend::detach_device(self.id, device)
    }
}

impl Drop for IommuDomain {
    fn drop(&mut self) {
        let _ = backend::free_domain(self.id);
    }
}
