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

//! Which PCI functions the driver takes for an AHCI HBA, from their ids and
//! the size of BAR5. Pure, so the host proofs hold the rule.

use crate::constants::regs::{PORT_BASE, PORT_STRIDE};

const PCI_CLASS_STORAGE: u8 = 0x01;
const PCI_SUBCLASS_RAID: u8 = 0x04;
const PCI_SUBCLASS_SATA: u8 = 0x06;
const VENDOR_INTEL: u16 = 0x8086;

/// The smallest ABAR the driver maps: the global registers and one port's
/// register block. An Intel PCH ABAR is commonly 2 KiB (ports 0 to 13), not
/// the 4 KiB QEMU's ICH9 has; the ports outside the mapped window are never
/// touched (`controller::ports_in_window`).
pub const MIN_ABAR_BYTES: u64 = PORT_BASE as u64 + PORT_STRIDE as u64;

/// Intel VMD device ids. A VMD reports class 01h subclass 04h like an RST
/// RAID-mode SATA controller, but it is a PCI domain, not an HBA, and its
/// BAR5 is no ABAR. Restated from the kernel's list
/// (src/hardware/inventory/vmd.rs), which no capsule can import; the proof
/// crate checks the two agree.
pub const INTEL_VMD_DEVICE_IDS: [u16; 13] = [
    0x201d, 0x28c0, 0x467f, 0x4c3d, 0x9a0b, 0xa77f, 0x7d0b, 0xad0b, 0xb06f, 0xb60b, 0xb07f, 0xd70b,
    0xd73b,
];

/// The function is one the driver may take as an AHCI HBA, by its ids:
/// any SATA controller (01h/06h, whatever its prog-if: AHCI is 01h, but
/// vendor-specific interfaces still put an ABAR in BAR5), and an Intel
/// RAID-mode controller (01h/04h), which Linux's ahci driver binds by
/// device id because Intel RST "RAID On" is AHCI underneath. Never an Intel
/// VMD.
pub fn is_ahci_function(pci_class: u8, pci_subclass: u8, vendor: u16, device: u16) -> bool {
    if pci_class != PCI_CLASS_STORAGE || is_intel_vmd(vendor, device) {
        return false;
    }
    match pci_subclass {
        PCI_SUBCLASS_SATA => true,
        PCI_SUBCLASS_RAID => vendor == VENDOR_INTEL,
        _ => false,
    }
}

pub fn is_intel_vmd(vendor: u16, device: u16) -> bool {
    vendor == VENDOR_INTEL && INTEL_VMD_DEVICE_IDS.contains(&device)
}

/// BAR5 is large enough to hold one port.
pub const fn abar_usable(size: u64) -> bool {
    size >= MIN_ABAR_BYTES
}
