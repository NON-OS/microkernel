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

//! Domain and mapping calls, routed by the vendor selected at boot.

use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DomainId, IommuError, IommuProtection};

use super::route::{route, Backend};

pub(crate) fn allocate_domain() -> Result<DomainId, IommuError> {
    match route(b"allocate_domain")? {
        Backend::Vtd => super::domain::allocate_domain(),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_domain::allocate_domain(),
    }
}

pub(crate) fn free_domain(id: DomainId) -> Result<(), IommuError> {
    match route(b"free_domain")? {
        Backend::Vtd => super::domain::free_domain(id),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_domain::free_domain(id),
    }
}

pub(crate) fn map(
    domain: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    match route(b"map")? {
        Backend::Vtd => super::mapping::map(domain, iova, phys, size, protection),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_domain::map(domain, iova, phys, size, protection),
    }
}

pub(crate) fn unmap(domain: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    match route(b"unmap")? {
        Backend::Vtd => super::mapping::unmap(domain, iova, size),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Backend::AmdVi => super::amd_domain::unmap(domain, iova, size),
    }
}
