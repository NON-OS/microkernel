// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! The Intel NIC rows by their capsules' id lists: each takes its own parts
//! and none takes another's.

use super::host_rows::pci;
use super::scope::admits;

const NIC: (u8, u8, u8) = (0x02, 0x00, 0x00);

#[test]
fn each_intel_nic_row_takes_its_own_parts() {
    assert!(admits("driver.e1000e_0", &pci(0x8086, 0x10D3, NIC)), "QEMU's e1000e");
    assert!(admits("driver.e1000e_0", &pci(0x8086, 0x550C, NIC)), "an ADP I219 in a run");
    assert!(admits("driver.igb_0", &pci(0x8086, 0x10C9, NIC)), "QEMU's igb");
    assert!(admits("driver.igc_0", &pci(0x8086, 0x15F3, NIC)), "an I225-V");
    assert!(admits("driver.igc_0", &pci(0x8086, 0x125C, NIC)), "an I226-V");
    assert!(admits("driver.ixgbe_0", &pci(0x8086, 0x1529, NIC)), "an 82599 in a run");
}

#[test]
fn no_intel_nic_row_takes_another_drivers_part() {
    assert!(
        !admits("driver.igc_0", &pci(0x8086, 0x15F4, NIC)),
        "an I219-LM, between its ranges, is e1000e's"
    );
    assert!(!admits("driver.e1000e_0", &pci(0x8086, 0x100E, NIC)), "an 8254x is e1000's");
    assert!(!admits("driver.igb_0", &pci(0x8086, 0x15F3, NIC)), "an I225 is igc's");
    assert!(!admits("driver.igc_0", &pci(0x8086, 0x1533, NIC)), "an I210 is igb's");
    assert!(!admits("driver.ixgbe_0", &pci(0x8086, 0x10B6, NIC)), "an 82598");
    assert!(!admits("driver.e1000_0", &pci(0x8086, 0x10D3, NIC)), "an 82574 is e1000e's");
}
