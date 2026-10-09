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

//! The same invalidation on every unit. The tables are shared, so a change to
//! them is stale in each unit's caches, not only the first one's.

use super::all::{invalidate_all, invalidate_iotlb};
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::report::units;

/// Context and IOTLB on every unit. Each unit is tried even after one fails,
/// so a single unit that stopped answering does not leave the others holding
/// translations the kernel has withdrawn; the first error is returned.
pub fn invalidate_all_units() -> Result<(), VtdError> {
    let mut outcome = Ok(());
    for info in units() {
        if let Err(e) = invalidate_all(&info.unit, info.ecap) {
            outcome = outcome.and(Err(e));
        }
    }
    outcome
}

/// IOTLB only, on every unit, as `invalidate_all_units`.
pub fn invalidate_iotlb_all_units() -> Result<(), VtdError> {
    let mut outcome = Ok(());
    for info in units() {
        if let Err(e) = invalidate_iotlb(&info.unit, info.ecap) {
            outcome = outcome.and(Err(e));
        }
    }
    outcome
}
