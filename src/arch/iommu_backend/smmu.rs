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

//! The SMMUv3 driver's backend, under the names `memory::iommu` calls.

pub(in crate::memory::iommu) use crate::arch::aarch64::iommu::smmu::backend::{
    allocate_domain, attach_device, capabilities, detach_device, free_domain, identity_devices,
    map, select_vendor, translates, unmap,
};
