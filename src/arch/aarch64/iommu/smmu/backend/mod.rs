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

//! The SMMUv3 behind the kernel's IOMMU contract.

mod devices;
mod domains;
mod mapping;
mod query;

pub use devices::{attach_device, detach_device};
pub use domains::{allocate_domain, free_domain};
pub use mapping::{map, unmap};
pub use query::{capabilities, select_vendor, translates};
