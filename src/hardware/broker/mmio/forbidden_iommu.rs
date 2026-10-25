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

//! Each IOMMU's register window, for forbidden_ranges.rs. A capsule with
//! those pages could turn translation off or point a domain at any memory.

extern crate alloc;

use alloc::vec::Vec;

#[cfg(target_arch = "x86_64")]
pub(super) fn push(out: &mut Vec<(u64, u64)>) {
    use crate::arch::x86_64::acpi::parser::other::{amd_iommu_bases, remap_unit_bases};
    /// A VT-d unit's registers, as unit/access.rs maps them.
    const VTD_WINDOW: u64 = 0x1000;
    /// An AMD-Vi unit's whole MMIO aperture, performance counters included.
    const AMD_WINDOW: u64 = 0x8_0000;
    let vtd = remap_unit_bases().into_iter().map(|b| (b, VTD_WINDOW));
    let amd = amd_iommu_bases().into_iter().map(|b| (b, AMD_WINDOW));
    // A base so high its window wraps is cut at the top, never dropped.
    out.extend(vtd.chain(amd).map(|(b, len)| (b, b.saturating_add(len))));
}

/// The unit the platform describes, in service or not: an SMMUv3 the device
/// tree names on aarch64 (arch::iommu_register_window).
#[cfg(not(target_arch = "x86_64"))]
pub(super) fn push(out: &mut Vec<(u64, u64)>) {
    if let Some((base, len)) = crate::arch::iommu_register_window() {
        out.push((base, base.saturating_add(len)));
    }
}
