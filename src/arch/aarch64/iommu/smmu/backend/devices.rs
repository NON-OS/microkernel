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

use crate::memory::iommu::{DeviceAddress, DomainId, IommuError};

use super::super::domain::slot;
use super::super::encode::ste;
use super::super::state::with;

/// Point the device's stream at the domain's CD. A device no stream of
/// this unit reaches answers `NoIommu`, so the broker leaves it on
/// physical addresses and counts it unconfined.
pub fn attach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    with(|smmu| {
        let sid = smmu.stream(device).ok_or(IommuError::NoIommu)?;
        let at = slot(domain).ok_or(IommuError::InvalidDomain)?;
        let mut d = smmu.domains[at].ok_or(IommuError::InvalidDomain)?;
        if !d.add(sid) {
            return Err(IommuError::DeviceAttachFailed);
        }
        smmu.strtab.write(sid, ste::stage1(d.cd), &mut smmu.cmdq, smmu.regs)?;
        smmu.domains[at] = Some(d);
        Ok(())
    })
}

/// Leave the device denied: its STE aborts, never bypasses.
pub fn detach_device(domain: DomainId, device: DeviceAddress) -> Result<(), IommuError> {
    with(|smmu| {
        let sid = smmu.stream(device).ok_or(IommuError::NoIommu)?;
        let at = slot(domain).ok_or(IommuError::InvalidDomain)?;
        let mut d = smmu.domains[at].ok_or(IommuError::InvalidDomain)?;
        smmu.strtab.write(sid, ste::abort(), &mut smmu.cmdq, smmu.regs)?;
        d.remove(sid);
        smmu.domains[at] = Some(d);
        Ok(())
    })
}
