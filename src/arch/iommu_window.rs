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

//! The register window of an IOMMU the platform describes, for the
//! broker's forbidden list on arches whose units are not ACPI's (x86_64
//! lists its VT-d and AMD-Vi units from the firmware tables itself).

/// Base and length of the platform's IOMMU registers, or `None`.
pub(crate) fn iommu_register_window() -> Option<(u64, u64)> {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::iommu::smmu::described_window();
    #[cfg(not(target_arch = "aarch64"))]
    None
}
