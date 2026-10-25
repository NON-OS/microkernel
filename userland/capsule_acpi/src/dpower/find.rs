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

//! The ACPI device a PCI function is: each inventoried device with an
//! _ADR, its path to the host bridge turned into a bus, device and
//! function through the bridges' secondary bus numbers.

use crate::boot::{evaluator, Acpi};
use crate::host::pci_function;
use nonos_aml::namespace::Id;
use nonos_aml::platform::device_pci_path;

pub fn device_at(acpi: &mut Acpi, bdf: (u16, u8, u8, u8)) -> Option<Id> {
    let Acpi { aml, host, devices, .. } = acpi;
    let with_adr = devices.iter().filter(|d| d.adr.is_some());
    for id in with_adr.map(|d| d.id).collect::<alloc::vec::Vec<_>>() {
        let path = device_pci_path(aml, &mut evaluator(host), id);
        if path.as_ref().and_then(pci_function) == Some(bdf) {
            return Some(id);
        }
    }
    None
}
