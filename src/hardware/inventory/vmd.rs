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

//! Intel Volume Management Device. With "RAID On" / VMD enabled in firmware,
//! the NVMe and SATA controllers disappear from the root bus and sit behind
//! this one function, which exposes a PCI domain of its own through its BARs.
//! It usually reports mass-storage class 01h subclass 04h (RAID), the same
//! as an Intel RST SATA controller in RAID mode, but it is no AHCI HBA: its
//! BAR5 is no ABAR, and a SATA driver poking it would fault or worse. These
//! are the device ids Linux's vmd driver binds (drivers/pci/controller/vmd.c).
//!
//! Pure, with no imports, so host proofs can include it by path; the AHCI
//! capsule restates the list and its proof crate checks the two agree.

const INTEL: u16 = 0x8086;

/// Intel VMD device ids.
pub const INTEL_VMD_DEVICE_IDS: [u16; 13] = [
    0x201d, 0x28c0, 0x467f, 0x4c3d, 0x9a0b, 0xa77f, 0x7d0b, 0xad0b, 0xb06f, 0xb60b, 0xb07f, 0xd70b,
    0xd73b,
];

/// Whether `vendor:device` is an Intel VMD controller.
pub const fn is_intel_vmd(vendor: u16, device: u16) -> bool {
    if vendor != INTEL {
        return false;
    }
    let mut i = 0;
    while i < INTEL_VMD_DEVICE_IDS.len() {
        if INTEL_VMD_DEVICE_IDS[i] == device {
            return true;
        }
        i += 1;
    }
    false
}
