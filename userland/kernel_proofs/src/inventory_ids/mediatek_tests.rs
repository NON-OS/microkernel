// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
// SPDX-License-Identifier: AGPL-3.0-or-later

//! The MediaTek Wi-Fi parts: the kernel's table is the mt7921e capsule's,
//! each of its parts is named for that capsule whatever its subclass, and a
//! MediaTek part no table names (MT7915, MT7925) is not taken for one.

use super::classify_network::classify_network;
use super::family::HardwareFamily::{NetworkMt7921, Unknown};
use super::mediatek_ids::MT7921_IDS;
use super::mt7921;

#[test]
fn the_kernels_table_is_the_capsules() {
    assert_eq!(MT7921_IDS, mt7921::PCI_IDS);
    for (v, d) in MT7921_IDS {
        assert!(mt7921::supported(v, d), "{v:04x}:{d:04x}");
    }
}

#[test]
fn each_part_is_the_mediatek_capsules_and_no_other_is() {
    for (v, d) in MT7921_IDS {
        for sub in [0x00, 0x80] {
            assert_eq!(classify_network(sub, v, d), NetworkMt7921, "{v:04x}:{d:04x}");
        }
    }
    for d in [0x7915, 0x7925, 0x7663, 0x7962] {
        assert_eq!(classify_network(0x80, 0x14c3, d), Unknown, "14c3:{d:04x}");
    }
    assert_eq!(classify_network(0x80, 0x0b48, 0x7961), Unknown, "ITTIM lists only the 7922");
}
