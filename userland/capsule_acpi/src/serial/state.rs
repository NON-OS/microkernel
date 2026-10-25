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

use alloc::vec::Vec;
use nonos_aml::namespace::Id;
use nonos_aml::serial::SerialDevice;

/// One device on a serial bus, with what the tables say it is and the
/// controllers its connection and its GpioInt (else first GPIO) name.
pub struct Peripheral {
    pub dev: SerialDevice,
    pub hid: Vec<u8>,
    pub hid_over_i2c: bool,
    /// HID over SPI's _DSM facts, read for a PNP0C51 device alone.
    pub hid_spi: Option<nonos_aml::serial::HidSpi>,
    pub controller: Option<Id>,
    /// The controller's PCI function, when its `_ADR` resolves to one; kept
    /// inside capsule_acpi (`bus_key`), never sent.
    pub controller_bdf: Option<super::bus_key::Bdf>,
    pub gpio_controller: Option<Id>,
}

#[derive(Default)]
pub struct Serial {
    pub list: Vec<Peripheral>,
}
