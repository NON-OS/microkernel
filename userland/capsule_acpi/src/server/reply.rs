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

//! Building the replies protocol.rs describes.

use super::caller::{group_of, owner, runs};
use super::inventory::{device, status};
use super::policy::{admits, SPI_HID};
use super::protocol::*;
use super::{brightness, lpit_wire, power_wire, smbios_wire, thermal_wire};
use crate::boot::Acpi;
use crate::sleep::prepare_sleep;
use alloc::vec::Vec;

/// The reply to `req`: status word, then body.
pub fn answer(acpi: &mut Acpi, req: &[u8], sender: u32) -> Vec<u8> {
    let index =
        |r: &[u8]| r.get(1..5).map(|b| u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize);
    let op = req.first().copied().unwrap_or(0);
    if !admits(op, sender, owner, group_of) {
        return EPERM.to_le_bytes().to_vec();
    }
    let spi_hid = op == OP_SERIAL && runs(sender, SPI_HID);
    let (code, body) = match req.first().copied() {
        Some(OP_STATUS) => (0, status(acpi)),
        Some(OP_DEVICE) => match index(req).and_then(|i| device(acpi, i)) {
            Some(b) => (0, b),
            None => (ENOENT, Vec::new()),
        },
        Some(OP_BRIGHTNESS) => brightness::get(acpi),
        Some(OP_SET_BRIGHTNESS) => brightness::set(acpi, req),
        Some(OP_PREPARE_SLEEP) => (prepare_sleep(acpi, req.get(1).copied()), Vec::new()),
        Some(OP_POWER) => power_wire::power(acpi),
        Some(OP_THERMAL) => thermal_wire::thermal(acpi),
        Some(OP_SMBIOS) => smbios_wire::smbios(acpi),
        Some(OP_SERIAL) => match index(req).and_then(|i| crate::serial::serial(acpi, i, spi_hid)) {
            Some(b) => (0, b),
            None => (ENOENT, Vec::new()),
        },
        Some(OP_DEVICE_POWER) => crate::dpower::device_power(acpi, req, sender),
        Some(OP_LEGACY) => crate::legacy::legacy(acpi, index(req)),
        Some(OP_PROPERTIES) => crate::props::properties(acpi, req, sender),
        Some(OP_GPIO_EVENT) => crate::aei::gpio_event(acpi, req, sender),
        Some(OP_TIME_ALARM) => crate::time_alarm::time_alarm(acpi),
        Some(OP_TIME_SET) => crate::time_alarm::time_set(acpi, req),
        Some(OP_QUIRKS) => crate::quirks::quirks(acpi, req, sender),
        Some(OP_LPIT) => lpit_wire::lpit_wire(),
        Some(OP_TPM_PPI) => crate::tpm_ppi::tpm_ppi(acpi),
        Some(OP_POWER_METER) => crate::power_meter::power_meter(acpi),
        Some(OP_SET_POWER_CAP) => crate::power_meter::set_power_cap(acpi, req),
        Some(OP_HOLDER) => super::holder::holder(req),
        Some(OP_PCI_FIRMWARE) => crate::pci_firmware::pci_firmware(acpi, req),
        _ => (EINVAL, Vec::new()),
    };
    let mut out = code.to_le_bytes().to_vec();
    out.extend_from_slice(&body);
    out
}
