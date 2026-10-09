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

//! Turning off MSI or MSI-X a function has on with no grant behind it: left
//! on by firmware's own drivers, or by a bind that failed part way. A bind
//! clears the other kinds before programming its own, since a function with
//! MSI and MSI-X both on is undefined (PCI Local Bus 3.0, 6.8) and one
//! sending messages does not assert INTx; Linux does the same in
//! pci_msi_init and pci_msix_init. The caller has checked that no grant of
//! either kind exists for the device.

use super::msi_program::clear as clear_msi;
use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{disable_msix, is_msi_enabled, is_msix_enabled};
use crate::hardware::broker::pci_index::PciHandle;

pub(super) fn msix_off(handle: &PciHandle) {
    let Some(msix) = handle.msix else { return };
    let cfg = ConfigSpace::new(handle.address);
    if is_msix_enabled(&cfg, &msix) == Ok(true) && disable_msix(&cfg, &msix).is_ok() {
        crate::log::info!("[IRQ] {} msi-x was on with no grant, turned off", handle.address);
    }
}

pub(super) fn msi_off(handle: &PciHandle) {
    let Some(msi) = handle.msi else { return };
    let cfg = ConfigSpace::new(handle.address);
    if is_msi_enabled(&cfg, &msi) == Ok(true) && clear_msi(&handle.address, &msi).is_ok() {
        crate::log::info!("[IRQ] {} msi was on with no grant, turned off", handle.address);
    }
}
