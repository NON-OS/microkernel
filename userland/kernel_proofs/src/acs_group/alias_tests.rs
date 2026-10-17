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

//! The requester ids a device behind a bridge to conventional PCI carries
//!: a PCIe-to-PCI bridge's secondary bus with device and function 0,
//! a legacy bridge's own id, one per bridge on the way up.

use super::acs_alias::aliases;
use super::acs_tree::{Bdf, Func};
use super::machine::*;

const TO_PCI: Bdf = (2, 3, 0);
const LEGACY: Bdf = (6, 4, 0);
const OLD_CARD: Bdf = (6, 1, 0);
const BEHIND_BOTH: Bdf = (7, 2, 0);

fn with_pci() -> Vec<Func> {
    let mut all = machine();
    all.push(Func { at: TO_PCI, buses: Some((6, 7)), conventional: true, alias: Some((6, 0, 0)) });
    all.push(Func { at: LEGACY, buses: Some((7, 7)), conventional: true, alias: Some(LEGACY) });
    let leaf = |at| Func { at, buses: None, conventional: false, alias: None };
    all.extend([leaf(OLD_CARD), leaf(BEHIND_BOTH)]);
    all
}

#[test]
fn a_pcie_device_carries_only_its_own_id() {
    assert!(aliases(NIC, &with_pci()).is_empty());
    assert!(aliases(SATA, &with_pci()).is_empty());
}

#[test]
fn a_card_behind_a_pcie_to_pci_bridge_carries_the_secondary_bus_id() {
    assert_eq!(aliases(OLD_CARD, &with_pci()), [(6, 0, 0)]);
}

#[test]
fn every_bridge_on_the_way_up_adds_its_alias_nearest_first() {
    assert_eq!(aliases(BEHIND_BOTH, &with_pci()), [LEGACY, (6, 0, 0)]);
}

#[test]
fn a_device_is_never_its_own_alias() {
    let mut all = with_pci();
    all.push(Func { at: (6, 0, 0), buses: None, conventional: false, alias: None });
    assert!(!aliases((6, 0, 0), &all).contains(&(6, 0, 0)));
}
