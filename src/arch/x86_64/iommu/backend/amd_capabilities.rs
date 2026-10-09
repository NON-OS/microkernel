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

//! What AMD-Vi guarantees while its units are in service. Leaves carry FC,
//! so every translated access snoops CPU caches, which is what snoop control
//! means on VT-d.

use crate::arch::x86_64::iommu::amd_vi::domain::MAX_DOMAINS;
use crate::arch::x86_64::iommu::amd_vi::is_enforcing;
use crate::memory::iommu::{IommuCapabilities, IommuVendor};

pub(super) fn amd_vi() -> IommuCapabilities {
    if !is_enforcing() {
        return IommuCapabilities::none_in_force(IommuVendor::AmdVi);
    }
    IommuCapabilities {
        vendor: IommuVendor::AmdVi,
        enforcing: true,
        address_width_bits: 48,
        interrupt_remapping: false,
        page_sizes: 1 << 12,
        snoop_control: true,
        domain_count: MAX_DOMAINS as u32,
    }
}
