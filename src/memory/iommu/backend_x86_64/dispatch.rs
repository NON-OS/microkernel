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

//! Every domain call, routed by the vendor selected at boot.

use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DeviceAddress, DomainId, IommuError, IommuProtection, IommuVendor};

use super::{refuse, select};

/*
 * On VT-d, and before selection has run, the call goes straight to the VT-d
 * backend, which gates itself on translation being in service. Once selection
 * has found no unit this kernel drives, the call is refused by name instead
 * of reaching tables no hardware walks.
 */
fn route(op: &'static [u8]) -> Result<(), IommuError> {
    match select::selected() {
        None | Some(IommuVendor::IntelVtd) => Ok(()),
        Some(IommuVendor::AmdVi) => Err(refuse::amd_vi(op)),
        Some(IommuVendor::Absent) => Err(refuse::absent(op)),
    }
}

pub(crate) fn allocate_domain() -> Result<DomainId, IommuError> {
    route(b"allocate_domain")?;
    super::domain::allocate_domain()
}

pub(crate) fn free_domain(id: DomainId) -> Result<(), IommuError> {
    route(b"free_domain")?;
    super::domain::free_domain(id)
}

pub(crate) fn map(
    domain: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    route(b"map")?;
    super::mapping::map(domain, iova, phys, size, protection)
}

pub(crate) fn unmap(domain: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    route(b"unmap")?;
    super::mapping::unmap(domain, iova, size)
}

pub(crate) fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    route(b"attach_device")?;
    super::device::attach_device(domain, device)
}

pub(crate) fn detach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    route(b"detach_device")?;
    super::device::detach_device(domain, device)
}
