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

//! Claims of the records the device tree names (`broker::described`, C's
//! records). Each kind is held to the service that drives it, once one does:
//! the xHCI controller to the xHCI capsule, which finds a device-tree
//! controller beside the PCI ones. No capsule drives the tree's SDHCI or
//! AHCI controllers yet, so they, and every other kind, are refused until a
//! unit names their driver (L59). Pure, so the proofs run it on the host;
//! `hold_ids.rs` holds the numbers below to the kernel's own.

use super::hold_rule::Hold;

pub(crate) const XHCI_SERVICE: &str = "driver.xhci0";

/// The device tree's bus kind.
pub(crate) const BUS_DEVICE_TREE: u8 = 4;
/// The `device` an xHCI controller's record names: its interface
/// (`described::Interface::Xhci`).
pub(crate) const TREE_XHCI: u16 = 2;

/// The hold on a record of the device tree, by the interface it names.
pub(crate) const fn hold_for_tree(device: u16) -> Hold {
    match device {
        TREE_XHCI => Hold::ForService(XHCI_SERVICE),
        _ => Hold::Refused,
    }
}
