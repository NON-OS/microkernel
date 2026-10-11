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
/// destroys the domain: every device still attached is denied, the domain's
/// translations are invalidated, and its table frames go back to the
/// allocator. A domain that cannot be freed keeps its id and its tables so
/// nothing reuses them, and the drop says so.
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
        backend::attach_device(self.id, device)?;
        super::posture::report_if_identity_moved();
        Ok(())
    }

    /// Reverse a previous `attach_device` for this device.
    pub fn detach_device(&self, device: DeviceAddress) -> Result<(), IommuError> {
        backend::detach_device(self.id, device)?;
        super::posture::report_if_identity_moved();
        Ok(())
    }
}

impl Drop for IommuDomain {
    fn drop(&mut self) {
        // Drop has no caller to hand the error to, so the log is where it goes;
        // the backend has already left the frames quarantined.
        if let Err(e) = backend::free_domain(self.id) {
            crate::log::warn!(
                "[IOMMU] domain {} not freed ({:?}); its id and tables stay quarantined",
                self.id.as_u16(),
                e
            );
        }
    }
}
