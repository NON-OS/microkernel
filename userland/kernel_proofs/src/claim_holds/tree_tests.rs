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

//! The device tree's records (`hold_tree`): its xHCI controller held to the
//! xHCI capsule, every other interface refused until a driver is named.

use super::hold_platform::{hold_for_platform, BUS_DEVICE_TREE, SERIAL, SERIAL_BUS};
use super::hold_rule::Hold;
use super::hold_tree::{hold_for_tree, TREE_XHCI, XHCI_SERVICE};

/// The class id an xHCI record carries (`described::Interface::class`).
const USB_HOST_XHCI: u32 = 0x0071;

#[test]
fn a_tree_xhci_controller_is_held_for_the_xhci_capsule_alone() {
    let held = Hold::ForService(XHCI_SERVICE);
    assert_eq!(XHCI_SERVICE, "driver.xhci0");
    assert_eq!(hold_for_tree(TREE_XHCI), held);
    // By its interface, whatever class fields the record carries.
    for class in [USB_HOST_XHCI, SERIAL, 0] {
        assert_eq!(hold_for_platform(BUS_DEVICE_TREE, class, 0x0c, 0, TREE_XHCI), Some(held));
    }
}

#[test]
fn every_other_tree_interface_is_refused_until_a_driver_is_named() {
    // SDHCI (1) and AHCI (3) have no capsule on the tree yet; the rest is unknown.
    for device in [0u16, 1, 3, 4, 5, 0x0303, 0xFFFF] {
        assert_eq!(hold_for_tree(device), Hold::Refused, "{device}");
        let hold = hold_for_platform(BUS_DEVICE_TREE, SERIAL, SERIAL_BUS, 0, device);
        assert_eq!(hold, Some(Hold::Refused), "{device}");
    }
}
