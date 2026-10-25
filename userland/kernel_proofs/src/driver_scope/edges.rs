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

//! The edges: a name with no row claims nothing, and the bus kinds
//! scope_match.rs restates are the broker's own.

use super::scope::{admits, Seen};

#[test]
fn the_bus_kinds_are_the_brokers() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../src/hardware/broker");
    let bus = std::fs::read_to_string(root.join("device/bus.rs")).expect("readable");
    let rule = std::fs::read_to_string(root.join("scope_match.rs")).expect("readable");
    assert!(bus.contains("const BUS_KIND_PCI: u8 = 1;") && rule.contains("const PCI: u8 = 1;"));
    assert!(bus.contains("const BUS_KIND_ACPI: u8 = 2;") && rule.contains("const ACPI: u8 = 2;"));
    assert!(bus.contains("const BUS_KIND_FDT: u8 = 4;") && rule.contains("const FDT: u8 = 4;"));
}

#[test]
fn a_service_with_no_row_claims_nothing() {
    for s in ["app.settings", "net.core", "driver.usb_hid0", "", "driver.e1000_0 "] {
        assert!(
            !admits(
                s,
                &Seen {
                    bus: 1,
                    vendor: 0x8086,
                    device: 0x100E,
                    class: 0,
                    pci_class: 2,
                    pci_sub: 0,
                    progif: 0
                }
            ),
            "{s:?}"
        );
        assert!(
            !admits(
                s,
                &Seen {
                    bus: 2,
                    vendor: 0,
                    device: 0,
                    class: 0x0060,
                    pci_class: 0,
                    pci_sub: 0,
                    progif: 0
                }
            ),
            "{s:?}"
        );
    }
}
