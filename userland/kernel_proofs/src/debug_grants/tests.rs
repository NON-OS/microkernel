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

/// Each driver: its kernel spawn, and its manifest.
const DRIVERS: &[(&str, &str, &str)] = &[
    (
        "rtl8169",
        include_str!("../../../../src/hardware/rtl8169_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_rtl8169/Capsule.mk"),
    ),
    (
        "xhci",
        include_str!("../../../../src/hardware/xhci_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_xhci/Capsule.mk"),
    ),
    (
        "usb_hid",
        include_str!("../../../../src/userspace/capsule_driver_usb_hid/spawn.rs"),
        include_str!("../../../capsule_driver_usb_hid/Capsule.mk"),
    ),
    (
        "ahci",
        include_str!("../../../../src/hardware/ahci_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_ahci/Capsule.mk"),
    ),
    (
        "e1000",
        include_str!("../../../../src/hardware/e1000_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_e1000/Capsule.mk"),
    ),
    (
        "iwlwifi",
        include_str!("../../../../src/hardware/iwlwifi_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_iwlwifi/Capsule.mk"),
    ),
    (
        "i2c_pci",
        include_str!("../../../../src/hardware/i2c_pci_capsule/spawn.rs"),
        include_str!("../../../capsule_driver_i2c_pci/Capsule.mk"),
    ),
];

/// The spawn's requested_caps expression, up to the field after it.
fn requested(spawn: &str) -> &str {
    let at = spawn.find("requested_caps:").expect("no requested_caps");
    let end = spawn[at..].find("debug_tag:").map_or(spawn.len(), |e| at + e);
    &spawn[at..end]
}

#[test]
fn each_spawn_folds_in_the_optional_debug() {
    for (name, spawn, _) in DRIVERS {
        assert!(
            requested(spawn).contains("| crate::capabilities::serial_debug_cap()"),
            "{name}'s spawn asks no Debug, so a development image's console lacks its lines"
        );
    }
}

#[test]
fn each_manifest_declares_debug_optional_and_nothing_else() {
    for (name, _, mk) in DRIVERS {
        let line = mk.lines().find(|l| l.starts_with("CAPSULE_OPTIONAL_CAPS"));
        let value = line.and_then(|l| l.split(":=").nth(1)).map(str::trim);
        assert_eq!(value, Some("0x100"), "{name}'s manifest");
    }
}

#[test]
fn no_spawn_asks_debug_outright() {
    for (name, spawn, _) in DRIVERS {
        assert!(
            !requested(spawn).contains("Capability::Debug"),
            "{name} asks Debug in every build"
        );
    }
}
