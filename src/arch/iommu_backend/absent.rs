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

use crate::memory::iommu::{DeviceAddress, IommuCapabilities, IommuVendor};

/// A build with no IOMMU backend reads no DMAR or IVRS table, so it has nothing to select.
pub(in crate::memory::iommu) fn select_vendor() -> IommuVendor {
    IommuVendor::Absent
}

pub(in crate::memory::iommu) fn capabilities() -> IommuCapabilities {
    IommuCapabilities::none_in_force(IommuVendor::Absent)
}

/// No backend translates anything, so no device may be handed an IOVA.
pub(in crate::memory::iommu) fn translates(_device: DeviceAddress) -> bool {
    false
}

/// With no backend there is no identity domain: every device reaches memory
/// directly, which the posture line already says with `none present`.
pub(in crate::memory::iommu) fn identity_devices() -> usize {
    0
}

// The domain calls, beside this file so each stays in bounds.
#[path = "absent_domain.rs"]
mod domain_calls;

pub(in crate::memory::iommu) use domain_calls::{
    allocate_domain, attach_device, detach_device, free_domain, map, unmap,
};
