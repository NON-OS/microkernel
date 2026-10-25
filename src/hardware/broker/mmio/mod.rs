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
pub mod bar_select;
mod cache;
pub mod fence;
mod forbidden;
mod forbidden_image;
mod forbidden_iommu;
mod forbidden_ranges;
mod map;
mod map_bar;
mod map_install;
mod map_notes;
mod map_record;
mod map_type;
mod memory_type;
pub mod msix_exclusion;
mod release;
mod take_back;
mod types;
pub mod window;

// Read by the IOMMU unit-base check, which only x86 builds (K11).
#[cfg(target_arch = "x86_64")]
pub(crate) use forbidden_ranges::ram as boot_ram_ranges;
pub use map::map_for_caller;
pub use release::{release_all_for_pid, release_for_device, revoke_for_device, unmap_grant};
pub(in crate::hardware::broker) use take_back::take_back;
pub use types::{MmioMapError, MmioMapRequest, MmioMapResult};
