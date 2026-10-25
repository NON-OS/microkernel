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


//! Whether a broker record is an I2C host driver.i2c_pci may drive: an Intel
//! PCI function in the LPSS I2C id ranges the capsule lists, or with the
//! class LPSS functions report (serial bus or signal processing, subclass
//! other), or the kernel's own ACPI record of an LPSS or AMD platform
//! controller (capsule_driver_i2c_pci discover/classify.rs). It is wider than
//! the capsule's own match and never narrower (kernel_proofs i2c_host), so a
//! host the capsule drives always gets it started. On a machine with none,
//! QEMU's q35 among them, neither it nor driver.i2c_hid, which asks it for its
//! touchpad, is started: they cost two spawns, and the HID capsule's first
//! call went to a host that had found nothing and was leaving.

const INTEL: u16 = 0x8086;
const BUS_PCI: u8 = 1;
const BUS_ACPI: u8 = 2;
/// The broker's class of an ACPI serial-bus controller (abi/driver_broker_abi.md).
const SERIAL: u32 = 0x0060;
const SERIAL_BUS: u8 = 0x0C;
const SIGNAL_PROCESSING: u8 = 0x11;
const OTHER: u8 = 0x80;

/// Intel LPSS I2C device ids, each range taken whole.
const LPSS_I2C_IDS: [(u16, u16); 35] = [
    (0x0AAC, 0x0ABA), (0x1AAC, 0x1ABA), (0x5AAC, 0x5ABA), (0x31AC, 0x31BA), (0x9D60, 0x9D65),
    (0xA160, 0xA162), (0xA2E0, 0xA2E3), (0xA3E0, 0xA3E3), (0x9DE8, 0x9DEB), (0x9DC5, 0x9DC6),
    (0xA368, 0xA36B), (0x02E8, 0x02EB), (0x02C5, 0x02C6), (0x06E8, 0x06EB), (0x4DE8, 0x4DEB),
    (0x4DC5, 0x4DC6), (0x34E8, 0x34EB), (0x34C5, 0x34C6), (0xA0E8, 0xA0EB), (0xA0C5, 0xA0C6),
    (0xA0D8, 0xA0D9), (0x43E8, 0x43EB), (0x43AD, 0x43AE), (0x43D8, 0x43D8), (0x51E8, 0x51EB),
    (0x51C5, 0x51C6), (0x51D8, 0x51D9), (0x54E8, 0x54EB), (0x54C5, 0x54C6), (0x7ACC, 0x7ACF),
    (0x7AFC, 0x7AFD), (0x7A4C, 0x7A4F), (0x7A7C, 0x7A7D), (0x7E78, 0x7E7B), (0x7E50, 0x7E51),
];

/// A record's bus kind, vendor, device id, broker class, PCI class and
/// subclass: whether driver.i2c_pci may take it.
pub fn i2c_host(bus: u8, vendor: u16, device: u16, class: u32, pci: (u8, u8)) -> bool {
    match bus {
        BUS_PCI if vendor == INTEL => {
            matches!(pci, (SERIAL_BUS | SIGNAL_PROCESSING, OTHER))
                || LPSS_I2C_IDS.iter().any(|&(lo, hi)| (lo..=hi).contains(&device))
        }
        BUS_ACPI => class == SERIAL && pci.0 == SERIAL_BUS,
        _ => false,
    }
}
