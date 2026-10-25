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
pub(super) const DRIVERS: &[(&str, &str, &str)] = &[
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
