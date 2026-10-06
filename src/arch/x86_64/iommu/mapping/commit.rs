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

use crate::arch::x86_64::iommu::tables::touched::Touched;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::invalidate::invalidate_iotlb_all_units;
use crate::arch::x86_64::iommu::unit::report::probed;

/// Make a run of leaf writes the unit's view before returning. An unmap is not
/// done until the IOTLB forgets it: the broker frees the frame next, and a
/// cached translation would let the device write into its next owner. A map
/// needs the same under caching mode, where a not-present entry is cached too.
pub(super) fn commit(touched: Touched) -> Result<(), VtdError> {
    touched.finish();
    probed().ok_or(VtdError::NotPresent)?;
    invalidate_iotlb_all_units()
}
