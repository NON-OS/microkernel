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

use crate::memory::addr::PhysAddr;
use crate::memory::iommu::{DomainId, IommuError, IommuProtection};

use super::super::domain::{asid, slot};
use super::super::encode::cmd;
use super::super::state::with;
use super::super::{pgtable, pgtable_unmap};

/// # Safety
///
/// As `IommuDomain::map`: the caller vouches the device may touch
/// `[phys, phys + size)` under `protection` until it is unmapped.
pub unsafe fn map(
    domain: DomainId,
    iova: u64,
    phys: PhysAddr,
    size: usize,
    protection: IommuProtection,
) -> Result<(), IommuError> {
    with(|smmu| {
        let at = slot(domain).ok_or(IommuError::InvalidDomain)?;
        let d = smmu.domains[at].ok_or(IommuError::InvalidDomain)?;
        let (read, write) = (protection.readable(), protection.writable());
        pgtable::map(d.ttb, iova, phys.as_u64(), size as u64, read, write)
    })
}

/// `Ok` only once the unit has dropped every TLB entry the domain's ASID
/// held, so the caller may scrub and reuse the frames behind the range.
pub fn unmap(domain: DomainId, iova: u64, size: usize) -> Result<(), IommuError> {
    with(|smmu| {
        let at = slot(domain).ok_or(IommuError::InvalidDomain)?;
        let d = smmu.domains[at].ok_or(IommuError::InvalidDomain)?;
        pgtable_unmap::unmap(d.ttb, iova, size as u64)?;
        smmu.cmdq.run(smmu.regs, &[cmd::tlbi_nh_asid(asid(at))])
    })
}
