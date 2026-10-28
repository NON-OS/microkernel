// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The RTL8852BE row by its capsule's ids (Linux v6.12's
//! `rtw89_8852be_id_table`, 10ec:b852 and 10ec:b85b): both parts, and
//! nothing else: not the RTL8852AE or RTL8852CE beside them, not another
//! Realtek card, not the same ids under another vendor; and no other Wi-Fi
//! driver takes them.

use super::host_rows::pci;
use super::scope::admits;

const WIFI: (u8, u8, u8) = (0x02, 0x80, 0x00);
const RTL: &str = "driver.rtl8852be0";

#[test]
fn the_rtl8852be_row_takes_both_parts() {
    for dev in [0xB852, 0xB85B] {
        assert!(admits(RTL, &pci(0x10EC, dev, WIFI)), "10ec:{dev:04x}");
    }
}

#[test]
fn the_rtl8852be_row_takes_nothing_else() {
    for dev in [0x8852, 0xC852, 0xB851, 0xB853, 0xB85A, 0xB85C, 0xC821, 0xC822, 0x8168] {
        assert!(!admits(RTL, &pci(0x10EC, dev, WIFI)), "10ec:{dev:04x}");
    }
    assert!(!admits(RTL, &pci(0x14C3, 0xB852, WIFI)), "another vendor's b852");
    for other in ["driver.rtl8821ce0", "driver.rtl8822ce0", "driver.mt7921e0", "driver.iwlwifi0"] {
        assert!(!admits(other, &pci(0x10EC, 0xB852, WIFI)), "{other} takes an RTL8852BE");
    }
}
