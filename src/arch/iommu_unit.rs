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

//! Whether the IOMMU a firmware description names is the one in service.

/// True when `unit` (the description's handle for it: an `iommus` phandle
/// on aarch64) is the unit this kernel brought up and drives.
pub(crate) fn iommu_unit_in_service(unit: u32) -> bool {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::iommu::smmu::in_service_phandle() == Some(unit);
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = unit;
        false
    }
}
