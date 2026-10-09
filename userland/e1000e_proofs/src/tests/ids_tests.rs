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

//! The device ID table: every ID lands in the family of the Linux board it
//! is listed under, no ID is in two lists, and nothing outside the lists is
//! claimed.

use crate::constants::ids::*;
use crate::constants::Family;

pub const BOARDS: [(&[u16], Family, &str); 9] = [
    (I82574, Family::I82574, "82574"),
    (I82583, Family::I82583, "82583"),
    (PCH_LPT, Family::PchLpt, "pch_lpt"),
    (PCH_SPT, Family::PchSpt, "pch_spt"),
    (PCH_CNP, Family::PchCnp, "pch_cnp"),
    (PCH_TGP, Family::PchTgp, "pch_tgp"),
    (PCH_ADP, Family::PchAdp, "pch_adp"),
    (PCH_MTP, Family::PchMtp, "pch_mtp"),
    (PCH_PTP, Family::PchPtp, "pch_ptp"),
];

#[test]
fn every_listed_id_classifies_to_its_board_and_only_once() {
    let mut all: Vec<u16> = Vec::new();
    for (ids, family, name) in BOARDS {
        for &id in ids {
            assert_eq!(Family::of(id), Some(family), "{id:#06x} is a {name} part");
            assert!(!all.contains(&id), "{id:#06x} is listed twice");
            all.push(id);
        }
        assert_eq!(family.name(), name);
        assert_eq!(family.is_pch(), family >= Family::PchLpt);
    }
    // e1000_pci_tbl in netdev.c: 3 for 82574/82583 and 61 for pch_lpt to pch_ptp.
    assert_eq!(all.len(), 64);
}

#[test]
fn the_linux_board_choices_that_cross_hw_h_names_are_kept() {
    assert_eq!(Family::of(0x10D3), Some(Family::I82574), "QEMU's -device e1000e");
    assert_eq!(Family::of(0x0D53), Some(Family::PchSpt), "PCH_CMP_I219_LM12 is SPT-bound");
    assert_eq!(Family::of(0x0D55), Some(Family::PchSpt), "PCH_CMP_I219_V12 is SPT-bound");
    assert_eq!(Family::of(0x0D4E), Some(Family::PchCnp), "PCH_CMP_I219_LM10 is CNP-bound");
    assert_eq!(Family::of(0x550E), Some(Family::PchMtp), "LNP runs as pch_mtp");
    assert_eq!(Family::of(0x550C), Some(Family::PchAdp), "PCH_ADP_I219_LM19");
    assert_eq!(Family::of(0x57B9), Some(Family::PchPtp), "PCH_NVL_I219_LM29");
}
