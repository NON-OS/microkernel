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

//! The PCI ids against Linux rtl8169_pci_tbl (r8169_main.c).

use crate::constants::pci::{REALTEK_VENDOR_ID, RTL8169_DEVICE_IDS};

#[test]
fn every_realtek_8169_8168_810x_and_8125_id_linux_lists_is_taken() {
    assert_eq!(REALTEK_VENDOR_ID, 0x10EC);
    for id in [0x2502u16, 0x2600, 0x3000, 0x8125, 0x8136, 0x8161, 0x8162, 0x8167, 0x8168, 0x8169] {
        assert!(RTL8169_DEVICE_IDS.contains(&id), "{id:#06x}");
    }
}

#[test]
fn the_5g_and_10g_parts_and_ids_other_drivers_own_are_not() {
    // 0x8126 RTL8126A, 0x8127 RTL8127A, 0x8129 (8139too too), 0x8139
    // (RTL8139, its own driver), 0xc821 (the RTL8821CE wireless part).
    for id in [0x8126u16, 0x8127, 0x8129, 0x8139, 0xC821] {
        assert!(!RTL8169_DEVICE_IDS.contains(&id), "{id:#06x}");
    }
}
