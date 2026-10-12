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

//! The register windows the kernel programs itself, which no OperationRegion
//! may reach whatever the memory map calls them.

use super::super::rule::Hardware;
use super::platform::fw as acpi;
use acpi::parser::other::{amd_iommu_bases, remap_unit_bases};

const PAGE: u64 = 0x1000;
/// The architectural local APIC base (Intel SDM vol. 3A, 11.4.1), protected
/// even when the MADT names another, since a relocated APIC can move back.
const LAPIC_DEFAULT: u64 = 0xFEE0_0000;
/// The largest register aperture AMD's IOMMU specification (48882) gives a
/// unit with its extended features.
const AMD_IOMMU_SPAN: u64 = 512 * 1024;
/// A VT-d unit's register set is a page or a few (DRHD Size field); sixteen
/// pages covers every unit reported so far and leaves nothing beside it.
const VTD_SPAN: u64 = 16 * PAGE;
/// One megabyte of configuration space per bus (PCI Firmware 3.3, 4.1.2).
const ECAM_BUS: u64 = 1 << 20;

pub(in super::super) fn protected(base: u64, end: u64) -> Option<Hardware> {
    let hit = |at: u64, len: u64| at < end && base < at.saturating_add(len);
    let lapic = acpi::lapic_address().unwrap_or(LAPIC_DEFAULT);
    if hit(lapic, PAGE) || hit(LAPIC_DEFAULT, PAGE) {
        return Some(Hardware::LocalApic);
    }
    if acpi::ioapics().iter().any(|io| hit(io.address, PAGE)) {
        return Some(Hardware::IoApic);
    }
    if acpi::hpet_address().is_some_and(|a| hit(a, PAGE)) {
        return Some(Hardware::Hpet);
    }
    let vtd = remap_unit_bases().iter().any(|&a| hit(a, VTD_SPAN));
    if vtd || amd_iommu_bases().iter().any(|&a| hit(a, AMD_IOMMU_SPAN)) {
        return Some(Hardware::Iommu);
    }
    let ecam = acpi::pcie_segments().iter().any(|s| {
        let first = s.base_address.saturating_add(u64::from(s.start_bus) * ECAM_BUS);
        let buses = u64::from(s.end_bus.saturating_sub(s.start_bus)) + 1;
        hit(first, buses * ECAM_BUS)
    });
    ecam.then_some(Hardware::Ecam)
}
