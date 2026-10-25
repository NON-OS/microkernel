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

//! The host an I2C-HID record names (K10g): the registered platform host
//! whose window the firmware's name carries, else the enumerated PCI
//! function at the name's `_ADR`.

extern crate alloc;

use alloc::vec::Vec;

use super::host_id::{host_device_id, pci_host_device_id, UNRESOLVED};
use crate::arch::firmware_table::devices::{I2cHostName, I2cPciHost};
use crate::hardware::broker::pci_index;

pub(super) struct Hosts {
    pub platform: Vec<I2cHostName>,
    pub registered: Vec<(u64, u64)>,
    pub pci: Vec<I2cPciHost>,
    pub functions: Vec<((u8, u8, u8), u64)>,
}

impl Hosts {
    pub(super) fn new(platform: Vec<I2cHostName>, registered: Vec<(u64, u64)>) -> Self {
        let pci = crate::arch::firmware_table::devices::enumerate_pci_i2c_hosts();
        let functions = pci_index::all()
            .into_iter()
            .filter(|h| h.address.segment == 0)
            .map(|h| ((h.address.bus, h.address.device, h.address.function), h.device_id))
            .collect();
        Hosts { platform, registered, pci, functions }
    }

    /// The device_id of the host `name` names, UNRESOLVED when none.
    pub(super) fn device_id(&self, name: [u8; 4]) -> u64 {
        let named = self.platform.iter().map(|h| (h.name, h.mmio_base));
        let id = host_device_id(name, named, &self.registered);
        if id != UNRESOLVED {
            return id;
        }
        pci_host_device_id(name, self.pci.iter().map(|h| (h.name, h.adr)), &self.functions)
    }
}
