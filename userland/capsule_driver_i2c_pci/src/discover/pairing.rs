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
//! Which controller an ACPI-declared device names, and so which it is
//! paired with: the probe order, the speed, the sweep and the reach follow.

use super::acpi_touchpad::AcpiTouchpad;
use super::found::Found;
use crate::constants::controller_index;

impl AcpiTouchpad {
    /// Whether `dev` is the controller this record's ResourceSource names:
    /// the one whose own record carries the id the kernel resolved, never
    /// one found by address. Left unresolved (the kernel resolves platform
    /// hosts by their window, and a PCI LPSS function has none in ACPI), a
    /// PCI function is named by "I2Cn", Intel's names for its LPSS functions
    /// in PCI function order. A resolved id always wins over the name.
    pub fn names(&self, dev: &Found) -> bool {
        match self.host {
            Some(id) => id == dev.device_id,
            None => {
                !dev.is_acpi
                    && self.controller_idx.is_some()
                    && controller_index(dev.pci_device) == self.controller_idx
            }
        }
    }

    /// The ResourceSource names no controller: no id, and no "I2Cn".
    pub fn names_none(&self) -> bool {
        self.host.is_none() && self.controller_idx.is_none()
    }

    /// The device may sit on `dev`: it names `dev`, or no controller at all.
    pub fn may_be_on(&self, dev: &Found) -> bool {
        self.names_none() || self.names(dev)
    }
}
