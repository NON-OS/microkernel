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

//! Which group members a claim reaches across a shared conventional bus
//! (acs_reach.rs `shared`), the only ones a port-only claim still
//! counts: never a sibling function or a peer past a port without ACS.

use super::acs_path::path;
use super::acs_reach::shared;
use super::acs_tree::Func;
use super::laptops::part;
use super::machine::*;

#[test]
fn q35s_smbus_is_a_sibling_of_sata_never_across_a_shared_bus() {
    let ich9 = |f: u8, d: u16| part((0, 0x1f, f), None, (0x8086, d), None).func;
    let all = vec![ich9(0, 0x2918), ich9(2, 0x2922), ich9(3, 0x2930)];
    assert!(shared(&path((0, 0x1f, 2), &all).unwrap()).is_empty(), "AHCI's");
    assert!(shared(&path((0, 0x1f, 3), &all).unwrap()).is_empty(), "SMBus's");
}

#[test]
fn a_device_on_a_shared_conventional_bus_is_shared() {
    let mut pci = machine();
    pci.push(Func { at: (2, 3, 0), buses: Some((6, 6)), conventional: true, alias: None });
    let leaf = |at| Func { at, buses: None, conventional: false, alias: None };
    pci.extend([leaf((6, 1, 0)), leaf((6, 2, 0))]);
    assert_eq!(shared(&path((6, 1, 0), &pci).unwrap()), [(6, 2, 0)]);
    assert!(shared(&path(NIC, &pci).unwrap()).is_empty(), "behind PCIe ports only");
}
