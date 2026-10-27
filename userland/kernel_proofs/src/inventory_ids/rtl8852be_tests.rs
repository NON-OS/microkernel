// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The RTL8852BE: the kernel's table is the capsule's own (`ids.rs`), both
//! its parts are named for that capsule, and the RTL8852CE beside it, which
//! no driver here takes, is held as a Realtek Wi-Fi part with no driver,
//! saying so, rather than taken for the 8852BE's.

#[path = "../../../../src/hardware/inventory/missing.rs"]
mod missing;
#[path = "../../../capsule_driver_rtl8852be/src/ids.rs"]
mod rtl8852be;

use super::classify_network::classify_network;
use super::family;
use super::family::HardwareFamily::{NetworkRealtekWifi, NetworkRtl8852be};
use super::realtek_ids::{
    RTL8169_DEVICE_IDS, RTL8821CE_DEVICE_IDS, RTL8822CE_DEVICE_IDS, RTL8852BE_DEVICE_IDS,
};

#[test]
fn the_kernels_table_is_the_capsules() {
    assert_eq!(RTL8852BE_DEVICE_IDS, rtl8852be::RTL8852BE_DEVICE_IDS);
    assert_eq!(rtl8852be::PCI_VENDOR_REALTEK, 0x10ec);
    // A BAR is a power of two, and must reach the indirect window at 0x40000.
    assert!(rtl8852be::BAR_MIN.is_power_of_two() && rtl8852be::BAR_MIN > 0x4_0000);
    for other in [&RTL8169_DEVICE_IDS[..], &RTL8821CE_DEVICE_IDS, &RTL8822CE_DEVICE_IDS] {
        assert!(!RTL8852BE_DEVICE_IDS.iter().any(|id| other.contains(id)));
    }
}

#[test]
fn both_parts_are_the_capsules_whatever_their_subclass() {
    for id in RTL8852BE_DEVICE_IDS {
        for sub in [0x00, 0x80] {
            assert_eq!(classify_network(sub, 0x10ec, id), NetworkRtl8852be, "{id:04x}");
        }
    }
    assert_eq!(missing::missing_path(NetworkRtl8852be), None);
}

#[test]
fn the_rtl8852ce_is_held_with_no_driver_and_says_so() {
    assert_eq!(classify_network(0x80, 0x10ec, 0xc852), NetworkRealtekWifi);
    let why = missing::missing_path(NetworkRealtekWifi).unwrap();
    assert!(why.contains("RTL8852CE") && why.starts_with("no driver"), "{why}");
}
