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

//! AMD-Vi bring-up: map every unit IVRS names, give every enumerated PCI
//! function an identity entry in one shared device table, deny every other
//! requester id, and put each unit in service. All or nothing: a unit that
//! cannot be reached leaves enforcement unclaimed.

use super::devtab::device_table;
use super::domain::{device_id, pass};
use super::enable::enable_unit;
use super::error::AmdViError;
use super::mmio::Unit;
use super::units::{record, set_enforcing, Units};
use crate::arch::x86_64::acpi::parser::other::{amd_iommu_bases, amd_iommu_spans};

/// Returns how many devices were given identity entries.
pub(super) fn bring_up() -> Result<usize, AmdViError> {
    let bases = amd_iommu_bases();
    if bases.is_empty() {
        return Err(AmdViError::NotPresent);
    }
    let mut mapped = Units::new();
    for base in bases.iter() {
        let unit = Unit::map(*base).ok_or(AmdViError::RegistersUnmappable)?;
        mapped.push(unit).map_err(|_| AmdViError::NotPresent)?;
    }
    let units = record(mapped);
    let table = device_table()?;

    // Entries before any unit reads the table: the disk, the USB controller
    // and the network card keep working the moment translation starts.
    let mut assigned = 0;
    for dev in crate::bus::pci::enumerate_devices().iter() {
        pass(device_id(dev.bus, dev.device, dev.function))?;
        assigned += 1;
    }
    // Ids firmware named one by one: the id a bridge's devices arrive under,
    // an ACPI device such as an eMMC host, an IOAPIC or HPET. None of them
    // need appear in PCI enumeration, and denying them breaks the device.
    for span in amd_iommu_spans().iter().filter(|s| s.named) {
        pass(span.first)?;
        assigned += 1;
    }
    for (index, unit) in units.iter().enumerate() {
        // SAFETY: eK@nonos.systems - `table` is allocated once and never
        // freed, and every enumerated device has its entry above.
        unsafe { enable_unit(index, unit, table)? };
    }
    set_enforcing();
    Ok(assigned)
}
