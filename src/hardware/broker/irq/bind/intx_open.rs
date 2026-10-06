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

//! Making sure the PCI function behind a bound line will assert it. Firmware
//! can leave a function with INTx Disable set, or with MSI or MSI-X on, and
//! either keeps the line silent; Linux clears the first in
//! do_pci_enable_device, and quiet.rs the others.

use super::quiet::{msi_off, msix_off};
use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::constants::CMD_INTERRUPT_DISABLE;
use crate::hardware::broker::pci_index;

const PCI_COMMAND: u16 = 0x04;

/// Nothing to do for a device with no PCI side-table entry (an ACPI one).
pub(super) fn open_intx(device_id: u64) {
    let Some(handle) = pci_index::lookup(device_id) else { return };
    let cfg = ConfigSpace::new(handle.address);
    msix_off(&handle);
    msi_off(&handle);
    if let Ok(cmd) = cfg.read16(PCI_COMMAND) {
        if cmd & CMD_INTERRUPT_DISABLE != 0
            && cfg.write16(PCI_COMMAND, cmd & !CMD_INTERRUPT_DISABLE).is_ok()
        {
            crate::log::info!("[IRQ] {} INTx was disabled, enabled", handle.address);
        }
    }
}
