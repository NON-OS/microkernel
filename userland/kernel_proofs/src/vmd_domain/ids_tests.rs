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

//! The VMD id table against Linux's vmd_ids (drivers/pci/controller/vmd.c):
//! every client part is recognised and honours VMCONFIG's bus range, and
//! the part whose bus range comes from BIOS data is not taken.

use super::domain::*;

/// Linux's VMD_FEATS_CLIENT entries, which carry VMD_FEAT_HAS_BUS_RESTRICTIONS.
const LINUX_CLIENT: [u16; 11] =
    [0x467f, 0x4c3d, 0xa77f, 0x7d0b, 0xad0b, 0x9a0b, 0xb60b, 0xb06f, 0xb07f, 0xd70b, 0xd73b];

#[test]
fn every_linux_client_vmd_is_recognised_with_its_bus_range() {
    for id in LINUX_CLIENT {
        assert!(is_intel_vmd(INTEL, id), "{id:04x}");
        assert_eq!(bus_start(id, 1, 2 << 8), Some(224), "{id:04x}");
        assert_eq!(bus_start(id, 1, 1 << 8), Some(128), "{id:04x}");
    }
}

#[test]
fn the_server_parts_keep_their_rules() {
    // 201d has no bus restriction registers: bus 0 whatever they read.
    assert!(is_intel_vmd(INTEL, 0x201d));
    assert_eq!(bus_start(0x201d, 1, 2 << 8), Some(0));
    assert_eq!(bus_start(0x28c0, 1, 2 << 8), Some(224));
    // 28c1 takes its bus range from BIOS data in MEMBAR2; not driven here.
    assert!(!is_intel_vmd(INTEL, 0x28c1));
}

#[test]
fn the_table_is_linux_client_and_server_parts_with_no_repeats() {
    let mut ids = VMD_DEVICE_IDS.to_vec();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), VMD_DEVICE_IDS.len());
    assert_eq!(ids.len(), LINUX_CLIENT.len() + 2);
}
