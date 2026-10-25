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

//! The domains bring-up puts enumerated functions in. No
//! function is identity mapped: one that firmware listed no reserved memory
//! for starts in an empty domain, so its DMA faults until a capsule claims
//! it; one with reserved memory gets those ranges and nothing else.

use crate::arch::x86_64::iommu::domain::create_domain;
use crate::arch::x86_64::iommu::globals::allocate_domain_id;
use crate::arch::x86_64::iommu::globals::state::STATE;
use crate::arch::x86_64::iommu::mapping::map_identity;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};

fn fresh() -> Result<(DomainId, u64), VtdError> {
    let id = DomainId::new(allocate_domain_id() as u16);
    create_domain(id)?;
    let state = STATE.lock();
    Ok((id, state.domains[id.as_u16() as usize].root))
}

/// The shared blocked domain: maps nothing.
pub(super) fn blocked_domain() -> Result<(DomainId, u64), VtdError> {
    fresh()
}

/// One device's own domain, its reserved `[base, end)` ranges mapped one to
/// one in 4 KiB leaves, read and write as the device's firmware use needs.
pub(super) fn reserved_domain(
    levels: u8,
    ranges: &[(u64, u64)],
) -> Result<(DomainId, u64), VtdError> {
    let (id, root) = fresh()?;
    for &range in ranges {
        map_identity(root, levels, range, 1)?;
    }
    Ok((id, root))
}
