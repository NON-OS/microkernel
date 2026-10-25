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

//! The arch's answer to which ranges a device must find mapped through its
//! IOMMU (`memory::iommu::reserved`). aarch64 reads them from the device
//! tree. No other arch answers here: x86_64's DMAR RMRRs take L60's own path
//! (`rmrr_regions`), so every device there has none in this answer.

use crate::memory::iommu::reserved::Reserved;
use crate::memory::iommu::DeviceAddress;

pub(crate) fn device_reserved_regions(device: DeviceAddress) -> Reserved {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::iommu::reserved::for_device(device);
    #[cfg(not(target_arch = "aarch64"))]
    {
        let _ = device;
        Reserved::NONE
    }
}

/// The regions a reserved range's physical span must lie inside.
pub(crate) fn reserved_memory_regions() -> &'static [(u64, u64)] {
    #[cfg(target_arch = "aarch64")]
    return crate::arch::aarch64::iommu::reserved::reserved_memory();
    #[cfg(not(target_arch = "aarch64"))]
    &[]
}
