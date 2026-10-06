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

use super::context::invalidate_context_global;
use super::iotlb::invalidate_iotlb_global;
use super::write_buffer::flush_write_buffer;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::access::RemapUnit;
use crate::arch::x86_64::iommu::unit::queue;

/// Both caches, in the order a table change requires. A unit with its queue
/// running takes them there: the spec leaves CCMD and the IOTLB register
/// undefined while QIE is set (VT-d 3.4, 6.5.2).
pub fn invalidate_all(unit: &RemapUnit, ecap: u64) -> Result<(), VtdError> {
    if queue::is_live(unit) {
        flush_write_buffer(unit)?;
        return queue::flush_caches(unit);
    }
    invalidate_context_global(unit)?;
    invalidate_iotlb_global(unit, ecap)
}

/// The IOTLB alone, by whichever path the unit is on.
pub fn invalidate_iotlb(unit: &RemapUnit, ecap: u64) -> Result<(), VtdError> {
    if queue::is_live(unit) {
        flush_write_buffer(unit)?;
        return queue::flush_iotlb(unit);
    }
    invalidate_iotlb_global(unit, ecap)
}
