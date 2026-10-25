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
    /// 0 no IOMMU, 1 Intel VT-d, 2 AMD-Vi, 3 Arm SMMUv3.
    pub iommu_vendor: u8,
    /// Whether the IOMMU translates with the kernel's tables.
    pub iommu_enforcing: bool,
    /// Mappings a device could reach with no IOMMU domain confining them. With
    /// enforcing set and this above zero, the unit is in service and DMA still
    /// goes around it.
    pub unconfined_grants: u32,
    /// Devices found and not claimed that the IOMMU still lets reach all of
    /// memory. DMA is held only when this and `unconfined_grants` are zero.
    pub identity_devices: u32,
    /// How long the firmware's event log is; zero when the loader carried none.
    pub log_len: u32,
    /// Whether the challenge came back unchanged. This is the anti-replay check
    /// and the one part of the document this capsule can verify on its own.
    pub challenge_echoed: bool,
    pub attest_len: u32,
    pub signature_len: u32,
}

impl Doc {
    /// The IOMMU the kernel named, or None when the machine has none. The
    /// parser refuses a vendor past SMMUv3, so nothing else reaches here.
    pub fn iommu_unit(&self) -> Option<&'static [u8]> {
        match self.iommu_vendor {
            1 => Some(b"VT-d"),
            2 => Some(b"AMD-Vi"),
            3 => Some(b"SMMUv3"),
            _ => None,
        }
    }

    /// Whether device DMA on the machine is confined: a unit enforcing, no
    /// grant around it, and no device it still lets reach all of memory.
    pub fn dma_held(&self) -> bool {
        self.iommu_enforcing && self.unconfined_grants == 0 && self.identity_devices == 0
    }
}
