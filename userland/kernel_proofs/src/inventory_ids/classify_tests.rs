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

//! What the inventory names each part, QEMU's among them.

use super::classify_network::classify_network;
use super::classify_storage::classify_storage;
use super::family::HardwareFamily::{self, *};
use super::intel_ethernet_ids::{
    E1000E_DEVICE_IDS, E1000_DEVICE_IDS, IGB_DEVICE_IDS, IGC_DEVICE_IDS, IXGBE_DEVICE_IDS,
};

#[test]
fn intel_ethernet_by_table() {
    let tables: [(&[u16], HardwareFamily); 5] = [
        (&E1000_DEVICE_IDS, NetworkE1000),
        (&E1000E_DEVICE_IDS, NetworkE1000e),
        (&IGB_DEVICE_IDS, NetworkIgb),
        (&IGC_DEVICE_IDS, NetworkIgc),
        (&IXGBE_DEVICE_IDS, NetworkIxgbe),
    ];
    for (ids, family) in tables {
        for &id in ids {
            assert_eq!(classify_network(0x00, 0x8086, id), family, "{id:04x}");
        }
    }
    // QEMU's e1000, e1000e and igb.
    assert_eq!(classify_network(0x00, 0x8086, 0x100e), NetworkE1000);
    assert_eq!(classify_network(0x00, 0x8086, 0x10d3), NetworkE1000e);
    assert_eq!(classify_network(0x00, 0x8086, 0x10c9), NetworkIgb);
    assert_eq!(classify_network(0x00, 0x8086, 0x1234), Unknown, "no driver's part");
    assert_eq!(classify_network(0x80, 0x8086, 0x2723), NetworkIwlwifi);
}

#[test]
fn realtek_by_table_and_wireless_by_subclass() {
    assert_eq!(classify_network(0x00, 0x10ec, 0x8139), NetworkRtl8139);
    for id in [0x8168, 0x8125, 0x2600] {
        assert_eq!(classify_network(0x00, 0x10ec, id), NetworkRtl8169, "{id:04x}");
    }
    assert_eq!(classify_network(0x80, 0x10ec, 0xc821), NetworkRtl8821ce);
    assert_eq!(classify_network(0x80, 0x10ec, 0xc822), NetworkRealtekWifi, "RTL8822CE");
    assert_eq!(classify_network(0x80, 0x10ec, 0x8852), NetworkRealtekWifi, "RTL8852AE");
    assert_eq!(classify_network(0x00, 0x10ec, 0x8126), Unknown, "RTL8126A, no driver yet");
}

#[test]
fn ufs_is_a_storage_family_of_its_own() {
    assert_eq!(classify_storage(0x09, 0x1b36, 0x0013), StorageUfs);
    assert_eq!(classify_storage(0x09, 0x8086, 0x98fa), StorageUfs);
    assert_eq!(classify_storage(0x08, 0x8086, 0xf1a8), StorageNvme);
}
