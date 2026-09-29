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

//! What a parsed document carries.

pub struct Doc {
    pub registry_root: [u8; 32],
    pub capsule_count: u32,
    /// Clear once any running capsule could not be recorded, which is the
    /// machine saying it no longer knows everything it is running.
    pub registry_complete: bool,
    /// 0 no IOMMU, 1 Intel VT-d, 2 AMD-Vi.
    pub iommu_vendor: u8,
    /// Whether the IOMMU translates with the kernel's tables.
    pub iommu_enforcing: bool,
    /// Mappings a device could reach with no IOMMU domain confining them. With
    /// enforcing set and this above zero, the unit is in service and DMA still
    /// goes around it.
    pub unconfined_grants: u32,
    /// Whether the challenge came back unchanged. This is the anti-replay check
    /// and the one part of the document this capsule can verify on its own.
    pub challenge_echoed: bool,
    pub attest_len: u32,
    pub signature_len: u32,
}
