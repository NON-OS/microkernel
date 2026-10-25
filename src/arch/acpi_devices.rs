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

//! The devices the firmware's ACPI namespace describes and PCI does not
//! enumerate: Intel LPSS I2C hosts, the HID-over-I2C devices on them, and
//! the GPIO communities their interrupts are wired to. The broker registers
//! them from here. Only the x86_64 AML reader finds them today; an Arm ACPI
//! machine describes the same devices and would answer here too.

#[cfg(target_arch = "x86_64")]
pub(crate) use crate::arch::x86_64::acpi::aml::{
    community_window, enumerate_gpio_controllers, enumerate_i2c_controllers, sbreg_from_bar0,
    GpioController, LpssController,
};
#[cfg(target_arch = "x86_64")]
pub(crate) use crate::arch::x86_64::acpi::devices::i2c::{
    enumerate_i2c_hid_devices, enumerate_pci_i2c_hosts, enumerate_platform_i2c_hosts, HidInterrupt,
    I2cHidDevice, I2cHidDeviceType, I2cHostName, I2cPciHost,
};
