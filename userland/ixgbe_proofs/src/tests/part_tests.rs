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
//! Which MAC each id is, and what differs by MAC: AUTOC kept and the
//! security block held on the 82599, NBASE-T on the X550.

use crate::facts::ids::{part_of, IXGBE_PARTS};
use crate::facts::parts::Part;

fn ids(of: Part) -> Vec<u16> {
    IXGBE_PARTS.iter().filter(|(_, p)| *p == of).map(|(id, _)| *id).collect()
}

#[test]
fn the_macs_are_as_observed() {
    let i82599 = [
        0x10F7, 0x1514, 0x1517, 0x10F8, 0x10F9, 0x10FB, 0x1529, 0x152A, 0x1557, 0x10FC, 0x151C,
        0x154D, 0x1558,
    ];
    assert_eq!(ids(Part::I82599), i82599);
    assert_eq!(ids(Part::X540), [0x1528, 0x1560]);
    assert_eq!(ids(Part::X550), [0x1563, 0x15D1]);
    assert_eq!(part_of(0x10FB), Some(Part::I82599), "the X520-DA2");
}

#[test]
fn what_differs_by_mac() {
    assert!(Part::I82599.is_82599() && !Part::X540.is_82599() && !Part::X550.is_82599());
    assert!(Part::X550.has_nbase_t() && !Part::X540.has_nbase_t() && !Part::I82599.has_nbase_t());
}

/// The 82598 has another register layout, the X550EM parts a firmware- or
/// external-PHY-managed link, the 82599 VF no physical function at all, and
/// the igb and igc parts their own drivers.
#[test]
fn parts_this_sequence_is_not_for_are_not_taken() {
    let not = [
        0x10B6, 0x10C6, 0x10C7, 0x10C8, 0x10DB, 0x10DD, 0x10EC, 0x10F1, 0x10F4,
        0x1508, // 82598
        0x15AA, 0x15AB, 0x15AC, 0x15AD, 0x15AE, 0x15C2, 0x15C8, 0x15E4, // X550EM
        0x10ED, 0x1515, 0x1565, // VFs
        0x10C9, 0x1533, 0x15F2, // igb, igc
    ];
    for id in not {
        assert_eq!(part_of(id), None, "{id:#06x}");
    }
}
