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

// The x86_64 VT-d / AMD-Vi backend lives in the arch tree
// (`src/arch/x86_64/iommu/backend/`) next to the hardware it drives;
// it is pulled in here by path so `memory::iommu` stays the single
// facade generic kernel code reaches through.
#[cfg(all(target_arch = "x86_64", feature = "nonos-arch-iommu"))]
#[path = "../../arch/x86_64/iommu/backend/mod.rs"]
mod inner;
#[cfg(not(all(target_arch = "x86_64", feature = "nonos-arch-iommu")))]
#[path = "backend_unsupported.rs"]
mod inner;

pub(super) use inner::{
    allocate_domain, attach_device, capabilities, detach_device, free_domain, map, select_vendor,
    translates, unmap,
};
