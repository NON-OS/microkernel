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

//! The broker's domain and mapping calls on AMD-Vi. Routing reaches here
//! only while every AMD unit is in service, so no call writes tables no
//! unit walks.

use crate::arch::x86_64::iommu::amd_vi::domain;
use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DomainId, IommuError, IommuProtection};

pub(super) fn allocate_domain() -> Result<DomainId, IommuError> {
    domain::create_domain().map(DomainId::new).map_err(|_| IommuError::DomainExhausted)
}

pub(super) fn free_domain(id: DomainId) -> Result<(), IommuError> {
    domain::destroy_domain(id.as_u16()).map_err(|_| IommuError::InvalidDomain)
}

pub(super) fn map(
    id: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    let access = (protection.read, protection.write);
    domain::map_range(id.as_u16(), iova, phys.as_u64(), size, access)
        .map_err(|_| IommuError::BackendFault)
}

pub(super) fn unmap(id: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    domain::unmap_range(id.as_u16(), iova, size).map_err(|_| IommuError::NotMapped)
}
