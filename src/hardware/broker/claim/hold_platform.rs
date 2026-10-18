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

//! Claims of the records firmware tables publish (L59). An ACPI or a device
//! tree record names what no PCI id does, and it was open to whichever
//! Driver-capped capsule asked first: once the I2C host driver had exited,
//! any capsule could take the touchpad's controller and its GPIO bank, and a
//! capsule holding the PS/2 ports reads every key. Each kind is held to the
//! service that drives it: a GPIO bank to the GPIO capsule or, in an image
//! without one, the I2C host driver that takes its pins itself; the SCI to
//! the ACPI capsule; the VMBus root to the VMBus core (`hold_virt`). A kind
//! not named here is refused until a unit names it. Pure, so the proofs run
//! it on the host; `hold_ids.rs` holds the numbers below to the kernel's own.

use super::hold_rule::Hold;
use super::hold_virt::{hold_for_virt, BUS_VIRT};

pub(crate) const I2C_HOST_SERVICE: &str = "driver.i2c_pci0";
pub(crate) const PS2_SERVICE: &str = "driver.ps2_kbd0";
pub(crate) const GPIO_SERVICE: &str = "driver.gpio0";
pub(crate) const ACPI_SERVICE: &str = "acpi";

pub(crate) const BUS_ACPI: u8 = 2;
/// The device tree's bus kind, reserved for C-13's records.
pub(crate) const BUS_DEVICE_TREE: u8 = 4;

pub(crate) const SERIAL: u32 = 0x0060;
pub(crate) const INPUT: u32 = 0x0040;
pub(crate) const GPIO_CTRL: u32 = 0x0080;
pub(crate) const OTHER: u32 = 0xFFFF;
/// The PCI base class an I2C host's record carries: serial bus controller.
pub(crate) const SERIAL_BUS: u8 = 0x0c;
pub(crate) const PNP: u16 = 0x0001;
pub(crate) const PS2_KBD: u16 = 0x0303;
pub(crate) const PS2_AUX: u16 = 0x0304;
/// PNP0C08, the ACPI SCI's own record (`acpi::sci`).
pub(crate) const PNP_ACPI: u16 = 0x0C08;

/// The hold on a record of `bus`, by its class id, PCI base class and
/// identity; None for a bus another rule keys.
pub(crate) const fn hold_for_platform(
    bus: u8,
    class: u32,
    pci_class: u8,
    vendor: u16,
    device: u16,
) -> Option<Hold> {
    match bus {
        BUS_ACPI => Some(match (class, pci_class, vendor, device) {
            (SERIAL, SERIAL_BUS, _, _) => Hold::ForService(I2C_HOST_SERVICE),
            (GPIO_CTRL, _, _, _) => Hold::ForEither(GPIO_SERVICE, I2C_HOST_SERVICE),
            (INPUT, _, PNP, PS2_KBD) | (INPUT, _, PNP, PS2_AUX) => Hold::ForService(PS2_SERVICE),
            (OTHER, _, PNP, PNP_ACPI) => Hold::ForService(ACPI_SERVICE),
            _ => Hold::Refused,
        }),
        BUS_VIRT => Some(hold_for_virt(vendor, device)),
        BUS_DEVICE_TREE => Some(Hold::Refused),
        _ => None,
    }
}
