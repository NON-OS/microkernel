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

//! An SD or eMMC host the ACPI capsule published, held for the SD host
//! capsule that serves it as it does a PCI one (B-102): the records the
//! kernel builds for F-76's three shapes, and no other record of its class.

use super::hold_platform::*;
use super::hold_rule::Hold;
use crate::hardware::broker::acpi::platform_record::record;
use crate::hardware::broker::acpi::platform_wire::{Platform, SD};
use crate::hardware::platform_sd::is_platform_sd_host;

const SPAWN: &str = include_str!("../../../../src/hardware/sdhci_capsule/spawn.rs");

fn sd_host(hid: &[u8]) -> Platform {
    let mut h = [0u8; 8];
    h[..hid.len()].copy_from_slice(hid);
    let windows = vec![(0xFED0_0000, 0x1000)];
    Platform { class: SD, irq_flags: 0, flags: 0, hid: h, uid: 0, gsiv: 0, windows, path: vec![] }
}

#[test]
fn an_acpi_sd_host_is_held_for_the_sd_host_capsule() {
    let held = Some(Hold::ForService(SDHCI_SERVICE));
    assert_eq!(SDHCI_SERVICE, "driver.sdhci0");
    assert!(SPAWN.contains("const SERVICE_NAME: &str = \"driver.sdhci0\";"), "its spawn's name");
    assert!(is_platform_sd_host(BUS_ACPI, SD_HOST, 0x05), "the class the inventory reads");
    for hid in [&b"80860F14"[..], b"80860F16", b"AMDI0040"] {
        let r = record(&sd_host(hid), 0);
        let hold = hold_for_platform(r.bus_kind, r.class, r.pci_class, r.vendor, r.device);
        assert_eq!((r.class, hold), (BLOCK, held), "{hid:?}");
    }
}

#[test]
fn no_other_record_of_its_class_or_shape_is_held_for_it() {
    // A BLOCK record without the SD host class is no SD host.
    for pci_class in [0u8, 0x01, SERIAL_BUS] {
        assert_eq!(hold_for_platform(BUS_ACPI, BLOCK, pci_class, 0, 0), Some(Hold::Refused));
    }
    // The SD host class under another class id.
    for class in [INPUT, OTHER, 0] {
        assert_eq!(hold_for_platform(BUS_ACPI, class, SD_HOST, 0, 0), Some(Hold::Refused));
    }
    // A device-tree SD host is keyed by its interface and still refused.
    assert_eq!(hold_for_platform(BUS_DEVICE_TREE, BLOCK, SD_HOST, 0, 1), Some(Hold::Refused));
}
