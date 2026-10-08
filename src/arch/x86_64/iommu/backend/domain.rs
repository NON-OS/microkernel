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

//! Domain lifetime on VT-d.

use crate::arch::x86_64::iommu::domain::DomainId as VtdDomainId;
use crate::arch::x86_64::iommu::domain::{create_domain, destroy_domain};
use crate::arch::x86_64::iommu::globals::allocate_domain_id;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::memory::iommu::{DomainId, IommuError};

use super::enforced;

/// Refuses before a unit is in service. A domain handed out then would accept
/// mappings and attachments that no hardware consults, which is the one thing
/// this backend must never let a caller believe it has.
pub(crate) fn allocate_domain() -> Result<DomainId, IommuError> {
    enforced::require()?;
    // A slot found free can be taken by a racing claim before it is created;
    // the loser looks again rather than failing a claim that had room.
    for _ in 0..4 {
        let raw = u16::try_from(allocate_domain_id()).map_err(|_| IommuError::DomainExhausted)?;
        match create_domain(VtdDomainId::new(raw)) {
            Ok(()) => return Ok(DomainId::new(raw)),
            Err(VtdError::DomainAlreadyExists) => continue,
            Err(_) => return Err(IommuError::DomainExhausted),
        }
    }
    Err(IommuError::DomainExhausted)
}

/// Teardown is deliberately ungated: a domain can only exist if allocation
/// succeeded, and refusing to free it would leak its page-table backing.
pub(crate) fn free_domain(id: DomainId) -> Result<(), IommuError> {
    destroy_domain(VtdDomainId::new(id.as_u16())).map_err(|_| IommuError::InvalidDomain)
}
