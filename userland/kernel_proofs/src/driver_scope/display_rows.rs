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

//! intel_display0's row: Intel graphics by class, whichever display subclass
//! firmware left it in, and nothing else, so it can claim no Intel NIC or
//! audio function and no other vendor's GPU.

use super::scope::{admits, Seen};

fn pci(vendor: u16, device: u16, class: (u8, u8, u8)) -> Seen {
    let (pci_class, pci_sub, progif) = class;
    Seen { bus: 1, vendor, device, class: 0, pci_class, pci_sub, progif }
}

const VGA: (u8, u8, u8) = (0x03, 0x00, 0x00);
const OTHER: (u8, u8, u8) = (0x03, 0x80, 0x00);
const ROW: &str = "driver.intel_display0";

#[test]
fn it_takes_intel_graphics_as_vga_or_other_display() {
    // 0x1916 is in the capsule's data/gen9_ids.txt.
    assert!(admits(ROW, &pci(0x8086, 0x1916, VGA)), "the boot display");
    assert!(admits(ROW, &pci(0x8086, 0x1916, OTHER)), "beside a discrete card");
}

#[test]
fn it_takes_nothing_else() {
    assert!(!admits(ROW, &pci(0x1234, 0x1111, VGA)), "bga's");
    assert!(!admits(ROW, &pci(0x1AF4, 0x1050, OTHER)), "virtio-gpu's");
    assert!(!admits(ROW, &pci(0x10DE, 0x1C82, VGA)), "another vendor's GPU");
    assert!(!admits(ROW, &pci(0x8086, 0x1916, (0x03, 0x02, 0x00))), "a 3D controller");
    assert!(!admits(ROW, &pci(0x8086, 0x10D3, (0x02, 0x00, 0x00))), "an Intel NIC");
    assert!(!admits(ROW, &pci(0x8086, 0x9D70, (0x04, 0x03, 0x00))), "Intel audio");
    let acpi = Seen { bus: 2, class: 0x0030, ..pci(0x8086, 0x1916, VGA) };
    assert!(!admits(ROW, &acpi), "class rows are PCI only");
}
