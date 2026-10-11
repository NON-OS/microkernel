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

use super::flush_rule::invalidates;
use crate::arch::x86_64::iommu::tables::touched::Touched;
use crate::arch::x86_64::iommu::types::{DomainId, VtdError};
use crate::arch::x86_64::iommu::unit::invalidate::{
    flush_write_buffers_all_units, invalidate_iotlb_domain_all_units,
};
use crate::arch::x86_64::iommu::unit::report::probed;

/// Make a run of leaf writes the unit's view before returning. An unmap is not
/// done until the IOTLB forgets it: the broker frees the frame next, and a
/// cached translation would let the device write into its next owner. A map
/// needs the same only under caching mode, where an empty entry is cached too.
pub(super) fn commit(touched: Touched, domain: DomainId, mapped: bool) -> Result<(), VtdError> {
    touched.finish();
    let unit = probed().ok_or(VtdError::NotPresent)?;
    if invalidates(mapped, unit.caching_mode) {
        invalidate_iotlb_domain_all_units(domain.as_u16())
    } else {
        flush_write_buffers_all_units()
    }
}
