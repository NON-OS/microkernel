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

use super::binding::DmaPosture;
use super::document::{IOMMU_AMD_VI, IOMMU_ARM_SMMU_V3, IOMMU_INTEL_VTD, IOMMU_NONE};
use crate::memory::iommu::{capabilities, identity_devices, unconfined_grants, IommuVendor};

/// Read once, so the values bound into the quote are the values the document
/// carries.
pub(super) fn dma_posture() -> DmaPosture {
    let caps = capabilities();
    let vendor = match caps.vendor {
        IommuVendor::Absent => IOMMU_NONE,
        IommuVendor::IntelVtd => IOMMU_INTEL_VTD,
        IommuVendor::AmdVi => IOMMU_AMD_VI,
        IommuVendor::ArmSmmuV3 => IOMMU_ARM_SMMU_V3,
    };
    let identity = u32::try_from(identity_devices()).unwrap_or(u32::MAX);
    DmaPosture {
        vendor,
        enforcing: caps.enforcing,
        unconfined_grants: unconfined_grants(),
        identity_devices: identity,
    }
}
