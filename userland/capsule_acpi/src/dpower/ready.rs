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

//! The wait after a move back to D0, for a PCI function: its host
//! bridge's function 9 duration when firmware offers one shorter than the
//! specification's, the specification's otherwise (`nonos_aml::pci_dsm`).
//! A device that is no PCI function, an I2C device or a platform
//! controller, waits nothing here; its own _PSx carries what it needs.
//! The answer to DEVICE_POWER, and boot's probe, come after the wait, so
//! no driver touches a function that is still recovering.

use crate::boot::{evaluator, Acpi};
use crate::pci_firmware::PciFirmware;
use crate::sys::acpi_pci::pci_read;
use crate::sys::clock::sleep_ms;
use nonos_aml::device_power::DState;
use nonos_aml::namespace::Id;
use nonos_aml::pci_dsm::{ready_wait_us, sleep_ms_for, wake_of, Wake};
use nonos_aml::platform::{device_pci_path, function_of, SECONDARY_BUS};

pub type Bdf = (u16, u8, u8, u8);

/// Only a real PCI-to-PCI bridge (header type 1) gives a secondary bus; a
/// USB controller's ports carry _ADRs too, and are not functions.
pub fn bridge_bus(f: Bdf) -> Option<u8> {
    let header = pci_read(f, 0x0E, 1).ok()? & 0x7F;
    (header == 1).then(|| pci_read(f, SECONDARY_BUS, 1).ok().map(|v| v as u8)).flatten()
}

/// Sleeps out the wait for a function already resolved; the wait in µs.
pub fn wait_ready(pf: &PciFirmware, f: Bdf, w: Wake) -> u32 {
    let us = ready_wait_us(pf.host_of(f).and_then(|h| h.facts.readiness), w);
    sleep_ms(sleep_ms_for(us));
    us
}

/// The wait after `dev` moved `from` -> `to`, slept out; None when the
/// move needs none or `dev` is no PCI function.
pub fn settle_after(acpi: &mut Acpi, dev: Id, from: DState, to: DState) -> Option<u32> {
    let w = wake_of(from, to)?;
    let path = device_pci_path(&mut acpi.aml, &mut evaluator(&mut acpi.host), dev)?;
    let f = function_of(&path, &mut bridge_bus)?;
    Some(wait_ready(&acpi.pci_firmware, f, w))
}
