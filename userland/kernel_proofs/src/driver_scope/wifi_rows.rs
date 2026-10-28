// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The MediaTek Wi-Fi row by its capsule's id list (Linux v6.12's
//! `mt7921_pci_device_table`): each listed part under its own vendor, and
//! nothing else: not a neighbour's id, not the ITTIM vendor's other parts,
//! not another Wi-Fi driver's card.

use super::host_rows::pci;
use super::scope::admits;

const WIFI: (u8, u8, u8) = (0x02, 0x80, 0x00);
const MT: &str = "driver.mt7921e0";

#[test]
fn the_mediatek_row_takes_each_listed_part() {
    for dev in [0x7961, 0x7922, 0x0608, 0x0616, 0x7920] {
        assert!(admits(MT, &pci(0x14C3, dev, WIFI)), "14c3:{dev:04x}");
    }
    assert!(admits(MT, &pci(0x0B48, 0x7922, WIFI)), "ITTIM's MT7922 module");
}

#[test]
fn the_mediatek_row_takes_nothing_else() {
    for dev in [0x7915, 0x7925, 0x7962, 0x0617, 0x7663] {
        assert!(!admits(MT, &pci(0x14C3, dev, WIFI)), "14c3:{dev:04x} is not in the table");
    }
    assert!(!admits(MT, &pci(0x0B48, 0x7961, WIFI)), "ITTIM lists only the 7922");
    assert!(!admits(MT, &pci(0x10EC, 0xC821, WIFI)), "an RTL8821CE");
    assert!(!admits("driver.rtl8821ce0", &pci(0x14C3, 0x0616, WIFI)), "nor it the MT7922");
    assert!(!admits("driver.iwlwifi0", &pci(0x14C3, 0x7961, WIFI)));
}
