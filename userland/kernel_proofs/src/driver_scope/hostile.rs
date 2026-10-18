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

//! A claim outside the claimant's row is refused, whatever the record says.

use super::scope::{admits, Seen};

fn pci(vendor: u16, device: u16, class: (u8, u8, u8)) -> Seen {
    let (pci_class, pci_sub, progif) = class;
    Seen { bus: 1, vendor, device, class: 0, pci_class, pci_sub, progif }
}

fn acpi(class: u32, vendor: u16, device: u16) -> Seen {
    Seen { bus: 2, vendor, device, class, pci_class: 0, pci_sub: 0, progif: 0 }
}

const NIC: (u8, u8, u8) = (0x02, 0x00, 0x00);
const NVME: (u8, u8, u8) = (0x01, 0x08, 0x02);

#[test]
fn each_driver_takes_its_own_devices() {
    assert!(admits("driver.e1000_0", &pci(0x8086, 0x100E, NIC)), "QEMU's e1000");
    assert!(admits("driver.iwlwifi0", &pci(0x8086, 0x08B3, (2, 0x80, 0))), "a 7260 in range");
    assert!(admits("driver.nvme0", &pci(0x144D, 0xA808, NVME)), "any vendor's NVMe");
    assert!(admits("driver.xhci0", &pci(0x1B21, 0x1242, (0x0C, 0x03, 0x30))), "an xHCI");
    assert!(admits("driver.ahci0", &pci(0x1022, 0x7901, (0x01, 0x06, 0x01))), "a SATA HBA");
    assert!(admits("driver.i2c_pci0", &pci(0x8086, 0xA0E8, (0x0C, 0x80, 0))), "TGL I2C0");
    assert!(admits("driver.i2c_pci0", &acpi(0x0060, 0, 0)), "an ACPI I2C controller");
    assert!(admits("driver.ps2_kbd0", &acpi(0x0040, 1, 0x0303)), "the PS/2 keyboard");
    assert!(admits("driver.virtio_rng", &pci(0x1AF4, 0x1044, (0xFF, 0, 0))), "virtio-rng");
}

#[test]
fn a_driver_cannot_claim_another_drivers_device() {
    assert!(!admits("driver.e1000_0", &pci(0x8086, 0x1533, NIC)), "an i210 is not 8254x");
    assert!(!admits("driver.e1000_0", &pci(0x144D, 0xA808, NVME)), "a NIC driver and a disk");
    assert!(!admits("driver.virtio_net0", &pci(0x1AF4, 0x1001, (1, 0, 0))), "virtio-blk");
    assert!(!admits("driver.rtl8139_0", &pci(0x10EC, 0x8168, NIC)), "an 8168");
    assert!(!admits("driver.nvme0", &pci(0x8086, 0x2822, (0x01, 0x04, 0))), "RAID");
    assert!(!admits("driver.xhci0", &pci(0x8086, 0x1E26, (0x0C, 0x03, 0x20))), "an EHCI");
    assert!(!admits("driver.ahci0", &pci(0x1022, 0x7905, (0x01, 0x04, 0))), "AMD RAID");
    assert!(!admits("driver.hda0", &pci(0x1022, 0x15E2, (0x04, 0x80, 0))), "AMD ACP");
    assert!(!admits("driver.ps2_kbd0", &acpi(0x0041, 0x15, 0x0001)), "an I2C touchpad");
    assert!(!admits("driver.iwlwifi0", &pci(0x8086, 0x08B5, (2, 0x80, 0))), "past 0x08B4");
}

#[test]
fn ids_are_held_to_their_bus_and_vendor() {
    assert!(!admits("driver.e1000_0", &acpi(0, 0x8086, 0x100E)), "an ACPI record with PCI ids");
    assert!(!admits("driver.e1000_0", &pci(0x10EC, 0x100E, NIC)), "the id under another vendor");
    assert!(!admits("driver.ps2_kbd0", &pci(1, 0x0303, (0x09, 0, 0))), "a PCI record with PNP ids");
    assert!(!admits("driver.nvme0", &acpi(0x0010, 0, 0)), "class rows are PCI only");
    // A PCI UART (07/00) is broker class SERIAL, as the ACPI I2C hosts are.
    let uart = Seen { class: 0x0060, ..pci(0x1415, 0xC158, (0x07, 0x00, 0x02)) };
    assert!(!admits("driver.i2c_pci0", &uart), "an ACPI row and a PCI record of its class");
}
