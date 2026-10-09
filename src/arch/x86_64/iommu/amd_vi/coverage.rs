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

//! Whether an AMD IOMMU in service translates a given PCI device: the units
//! are enforcing and an IVHD device entry covers the device's requester id.
//! A device no unit covers would take an IOVA for a physical address, so the
//! broker leaves it on its physical address, counted as unconfined.

use super::domain::device_id;
use super::units::is_enforcing;
use crate::arch::x86_64::acpi::parser::other::amd_iommu_spans;
use crate::arch::x86_64::acpi::parser::other::ivhd_scope::covers;

pub fn unit_covers(bus: u8, device: u8, function: u8) -> bool {
    is_enforcing() && covers(&amd_iommu_spans(), device_id(bus, device, function))
}
