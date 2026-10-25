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

//! What each device stood for, kept while it can still be asked. A bridge
//! among its functions is kept with the buses it leads to, so what is
//! behind it can be taken away once it can no longer be asked (PCI-to-PCI
//! Bridge Architecture 1.2: secondary bus at 0x19, subordinate at 0x1A).

use super::functions::answering;
use super::state::{Bridge, Hotplug};
use crate::boot::Acpi;
use crate::dpower::bridge_bus;
use crate::sys::acpi_pci::pci_read;
use alloc::vec::Vec;
use nonos_aml::namespace::Id;

const SUBORDINATE_BUS: u16 = 0x1A;

/// What `id` stands for now, replacing what was remembered.
pub fn remember(hp: &mut Hotplug, id: Id, bdfs: Vec<super::state::Bdf>) {
    hp.forget(id);
    for &f in &bdfs {
        let Some(secondary) = bridge_bus(f) else { continue };
        let subordinate = pci_read(f, SUBORDINATE_BUS, 1).map_or(secondary, |v| v as u8);
        hp.bridges.push(Bridge { bdf: f, secondary, subordinate });
    }
    if !bdfs.is_empty() {
        hp.known.push((id, bdfs));
    }
}

/// At start: every device firmware describes that stands for a function now.
pub fn learn(acpi: &mut Acpi) {
    let ids: Vec<Id> = acpi.devices.iter().filter(|d| d.adr.is_some()).map(|d| d.id).collect();
    for id in ids {
        let bdfs = answering(acpi, id);
        remember(&mut acpi.hotplug, id, bdfs);
    }
}
