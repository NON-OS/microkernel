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

//! Under remapping (VT-d with CFI clear, or AMD-Vi with IV set) the units
//! drop a compatibility format IO-APIC entry no remapping entry backs. The
//! kernel's own writers of one refuse rather than arm a line that never
//! fires; a capsule's line goes through program_remapped or
//! program_amd instead.

use super::super::error::IoApicResult;

pub fn compat_allowed() -> IoApicResult<()> {
    #[cfg(feature = "nonos-iommu-intremap")]
    if crate::arch::x86_64::iommu::remap::is_remapping() {
        return Err(super::super::error::IoApicError::NotRemapped);
    }
    #[cfg(feature = "nonos-iommu-amdvi")]
    if crate::arch::x86_64::iommu::amd_vi::intr::is_remapping() {
        return Err(super::super::error::IoApicError::NotRemapped);
    }
    Ok(())
}
