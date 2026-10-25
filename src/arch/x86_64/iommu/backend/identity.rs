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

//! How many devices the selected unit still lets reach all of memory: none
//! on VT-d, whose bring-up maps no identity domain (K2), and the passthrough
//! entries counted on AMD-Vi. Before selection, or with no unit, none.

use crate::memory::iommu::IommuVendor;

use super::select;

pub(crate) fn identity_devices() -> usize {
    match select::selected() {
        Some(IommuVendor::IntelVtd) => crate::arch::x86_64::iommu::globals::identity_devices(),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Some(IommuVendor::AmdVi) => crate::arch::x86_64::iommu::amd_vi::domain::identity_devices(),
        _ => 0,
    }
}
