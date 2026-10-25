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

//! A PCI function added to the table after the boot scan (the ACPI broker's).

use super::init::handle_for;
use super::pci_record::record_from_pci;
use super::state::{next_id, TABLE};
use crate::drivers::pci::types::PciDevice;
use crate::hardware::broker::pci_index;

/// A PCI function that answered only after firmware powered it up (`_PS0`),
/// added after the boot scan under the next free id, with its index handle.
#[allow(dead_code)] // the ACPI broker's alone, which only x86 builds
/// `disabled` marks a function no driver may claim (its BARs unassigned).
pub(in crate::hardware::broker) fn add_pci_function(dev: &PciDevice, disabled: bool) -> u64 {
    let external = super::external::flag_for(&dev.address);
    let mut table = TABLE.write();
    let id = next_id(table.iter().map(|r| r.device_id).max(), 0);
    let mut record = record_from_pci(id, dev);
    record.flags |= external;
    if disabled {
        record.flags |= crate::hardware::broker::DEVICE_FLAG_DISABLED;
    }
    table.push(record);
    pci_index::push(handle_for(id, dev));
    id
}
