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

//! The guarantees the selected IOMMU gives right now.

use crate::arch::x86_64::acpi::parser::other::{dmar_address_width, units_left_off};
use crate::arch::x86_64::iommu::globals::{is_enforcing, page_levels};
use crate::arch::x86_64::iommu::regs::cap::snoop_control;
#[cfg(feature = "nonos-iommu-intremap")]
use crate::arch::x86_64::iommu::remap::is_remapping;
use crate::arch::x86_64::iommu::types::{MAX_VTD_DOMAINS, PAGE_SIZE_4K};
use crate::arch::x86_64::iommu::unit::report::probed;
use crate::memory::iommu::{IommuCapabilities, IommuVendor};

use super::select;

/// Before selection nothing has been checked, so nothing is claimed.
pub(crate) fn capabilities() -> IommuCapabilities {
    match select::selected() {
        Some(IommuVendor::IntelVtd) => vtd(),
        #[cfg(feature = "nonos-iommu-amdvi")]
        Some(IommuVendor::AmdVi) => super::amd_capabilities::amd_vi(),
        Some(vendor) => IommuCapabilities::none_in_force(vendor),
        None => IommuCapabilities::none_in_force(IommuVendor::Absent),
    }
}

/*
 * Bring-up programs every segment 0 unit it holds. A unit left off (another
 * segment, past eight, past an unreadable DMAR structure) translates nothing,
 * and devices behind it reach memory directly, so nothing is claimed. The
 * width is what the domains' depth reaches, cut to what the unit accepts and
 * to DMAR's host address width (K11). Pages are 4 KiB: map_range maps no more.
 */
fn vtd() -> IommuCapabilities {
    let none = IommuCapabilities::none_in_force(IommuVendor::IntelVtd);
    let (Some(info), Some(levels)) = (probed(), page_levels()) else {
        return none;
    };
    if !is_enforcing() || units_left_off() != 0 {
        return none;
    }
    let haw = dmar_address_width().unwrap_or(u8::MAX);
    let Some(reach) = levels.checked_mul(9).and_then(|bits| bits.checked_add(12)) else {
        return none;
    };
    IommuCapabilities {
        vendor: IommuVendor::IntelVtd,
        enforcing: true,
        address_width_bits: reach.min(info.max_address_width).min(haw),
        interrupt_remapping: is_remapping(),
        page_sizes: PAGE_SIZE_4K as u64,
        snoop_control: snoop_control(info.ecap),
        domain_count: info.domains.min(MAX_VTD_DOMAINS as u32),
    }
}

/// A kernel built without interrupt remapping never turns it on.
#[cfg(not(feature = "nonos-iommu-intremap"))]
fn is_remapping() -> bool {
    false
}
