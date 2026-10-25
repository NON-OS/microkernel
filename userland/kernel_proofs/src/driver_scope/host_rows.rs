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

//! The rows of the drivers main added beside K6f (the Intel NICs are in
//! nic_rows.rs): the USB, SD, UFS, SMBus and SPI hosts by class, the
//! device tree's xHCI by its interface, and bga by its one id; and the GPIO
//! communities gpio0's alone, now that i2c_pci0 asks gpio0 for its doorbell.

use super::scope::{admits, Seen};

pub(super) fn pci(vendor: u16, device: u16, class: (u8, u8, u8)) -> Seen {
    let (pci_class, pci_sub, progif) = class;
    Seen { bus: 1, vendor, device, class: 0, pci_class, pci_sub, progif }
}

fn fdt(device: u16, class: (u8, u8, u8)) -> Seen {
    Seen { bus: 4, device, ..pci(0, device, class) }
}

const SD: (u8, u8, u8) = (0x08, 0x05, 0x01);

#[test]
fn each_new_driver_takes_its_own_devices() {
    assert!(admits("driver.bga0", &pci(0x1234, 0x1111, (0x03, 0x00, 0x00))), "bga");
    assert!(admits("driver.sdhci0", &pci(0x1B36, 0x0007, SD)), "QEMU's SD host");
    assert!(admits("driver.sdhci0", &pci(0x8086, 0x9D2B, (0x08, 0x05, 0x01))), "an Intel eMMC");
    let acpi_sd = Seen { bus: 2, class: 0x0010, ..pci(0, 0, (0x08, 0x05, 0)) };
    assert!(admits("driver.sdhci0", &acpi_sd), "an SD host firmware names");
    assert!(admits("driver.ufs0", &pci(0x1B36, 0x0013, (0x01, 0x09, 0x00))), "QEMU's UFS");
    assert!(admits("driver.ehci0", &pci(0x8086, 0x1E26, (0x0C, 0x03, 0x20))), "an EHCI");
    assert!(admits("driver.ohci0", &pci(0x106B, 0x003F, (0x0C, 0x03, 0x10))), "an OHCI");
    assert!(admits("driver.uhci0", &pci(0x8086, 0x7020, (0x0C, 0x03, 0x00))), "PIIX3 UHCI");
    assert!(admits("driver.smbus0", &pci(0x8086, 0x2930, (0x0C, 0x05, 0x00))), "ICH9 SMBus");
    assert!(admits("driver.spi0", &pci(0x8086, 0xA0AA, (0x0C, 0x80, 0x00))), "an LPSS SPI");
    assert!(admits("driver.xhci0", &fdt(2, (0x0C, 0x03, 0x30))), "the device tree's xHCI");
}

#[test]
fn the_new_rows_hold_to_their_own() {
    assert!(!admits("driver.ahci0", &pci(0x1B36, 0x0007, SD)), "SD left the AHCI capsule");
    assert!(!admits("driver.sdhci0", &pci(0x8086, 0x2922, (0x01, 0x06, 0x01))), "ICH9 SATA");
    assert!(!admits("driver.ufs0", &pci(0x144D, 0xA808, (0x01, 0x08, 0x02))), "an NVMe");
    assert!(!admits("driver.ehci0", &pci(0x1B21, 0x1242, (0x0C, 0x03, 0x30))), "an xHCI");
    assert!(!admits("driver.smbus0", &pci(0x1022, 0x790B, (0x0C, 0x05, 0x00))), "AMD's");
    assert!(!admits("driver.spi0", &pci(0x8086, 0xA0E8, (0x0C, 0x03, 0x30))), "another class");
    assert!(!admits("driver.bga0", &pci(0x1AF4, 0x1050, (0x03, 0x80, 0x00))), "virtio-gpu");
    assert!(!admits("driver.xhci0", &fdt(1, (0x0C, 0x03, 0x30))), "an SDHCI interface");
    assert!(!admits("driver.sdhci0", &fdt(1, SD)), "the capsule takes no tree host");
    assert!(!admits("driver.ahci0", &fdt(3, (0x01, 0x06, 0x01))), "nor a tree AHCI");
    let community = Seen { bus: 2, class: 0x0080, ..pci(0, 0, (0, 0, 0)) };
    assert!(admits("driver.gpio0", &community) && !admits("driver.i2c_pci0", &community));
}
