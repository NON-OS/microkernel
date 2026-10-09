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

#[cfg(feature = "nonos-iommu-amdvi")]
mod amd_capabilities;
#[cfg(feature = "nonos-iommu-amdvi")]
mod amd_device;
#[cfg(feature = "nonos-iommu-amdvi")]
mod amd_domain;
mod capabilities;
mod device;
mod dispatch;
mod dispatch_device;
mod domain;
mod enforced;
mod mapping;
mod refuse;
mod route;
mod select;

pub(crate) use capabilities::capabilities;
pub(crate) use dispatch::{allocate_domain, free_domain, map, unmap};
pub(crate) use dispatch_device::{attach_device, detach_device, translates};
pub(crate) use select::select_vendor;
