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

use super::assign::assign_enumerated;
use super::assign_named::assign_named;
use super::domain::blocked_domain;
use crate::arch::x86_64::iommu::globals::{set_enforcing, set_page_levels, set_snoop_control};
use crate::arch::x86_64::iommu::regs::offsets;
use crate::arch::x86_64::iommu::tables::root::root_table;
use crate::arch::x86_64::iommu::types::VtdError;
use crate::arch::x86_64::iommu::unit::enable::bring_into_service;
use crate::arch::x86_64::iommu::unit::report::{probed, units};

/// Program the tables, assign the enumerated devices, then enable. Every
/// device the kernel knows about has a context entry before translation
/// starts: in the blocked domain, or its own reserved memory (domain.rs). A
/// driver reaches a device only once its capsule claims it; anything not
/// enumerated, including a card plugged in later, is denied.
///
/// Every unit is pointed at the same root table. A client Intel machine has
/// one unit for the integrated graphics and one for everything else; the disk
/// and the USB controller sit behind the second, so programming only the first
/// would hand their capsules device addresses no hardware translates.
pub fn bring_up() -> Result<(usize, usize), VtdError> {
    let info = probed().ok_or(VtdError::NotPresent)?;
    // Checked for every unit before any is touched: one left enabled by
    // firmware cannot be taken over, and enabling the rest around it would
    // leave the machine half under tables nobody reasons about.
    if units().iter().any(|u| u.unit.read32(offsets::GSTS) & offsets::GSTS_TES != 0) {
        return Err(VtdError::FirmwareOwnsUnit);
    }
    let levels = info.levels.page_table_levels();

    set_page_levels(levels);
    set_snoop_control(info.ecap);
    let root = root_table()?;

    let blocked = blocked_domain()?;
    let aw = info.levels.context_aw();
    let (assigned, reserved) = assign_enumerated(blocked, levels, aw)?;
    // Devices only the ACPI namespace declares start blocked too (K2).
    let named = assign_named(blocked.1, blocked.0, aw)?;

    // A unit that fails after others came up leaves those translating through
    // the blocked and reserved domains; enforcement is simply not claimed.
    for unit in units() {
        // SAFETY: eK@nonos.systems - `root` is a table this module allocated
        // and never frees, so the unit may walk it by physical address for the
        // life of the kernel. Every enumerated device already has a context
        // entry in it.
        unsafe {
            bring_into_service(&unit.unit, root, unit.ecap)?;
        }
    }
    super::firmware::release_protected_regions_all();
    set_enforcing();
    Ok((assigned + named, reserved))
}
