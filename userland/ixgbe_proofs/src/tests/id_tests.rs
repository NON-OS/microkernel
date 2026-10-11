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
//! The id table against the IXGBE_DEV_ID_* names ixgbe_type.h gives them,
//! in ixgbe_pci_tbl's order for the MACs this driver takes.

use crate::constants::pci::{INTEL_VENDOR_ID, IXGBE_PARTS};

const IXGBE_PCI_TBL: [(u16, &str); 17] = [
    (0x10F7, "82599_KX4"),
    (0x1514, "82599_KX4_MEZZ"),
    (0x1517, "82599_KR"),
    (0x10F8, "82599_COMBO_BACKPLANE"),
    (0x10F9, "82599_CX4"),
    (0x10FB, "82599_SFP"),
    (0x1529, "82599_SFP_FCOE"),
    (0x152A, "82599_BACKPLANE_FCOE"),
    (0x1557, "82599_SFP_EM"),
    (0x10FC, "82599_XAUI_LOM"),
    (0x151C, "82599_T3_LOM"),
    (0x154D, "82599_SFP_SF2"),
    (0x1558, "82599_QSFP_SF_QP"),
    (0x1528, "X540T"),
    (0x1560, "X540T1"),
    (0x1563, "X550T"),
    (0x15D1, "X550T1"),
];

#[test]
fn every_listed_id_is_bound_in_order_and_nothing_else() {
    let ids: Vec<u16> = IXGBE_PARTS.iter().map(|(id, _)| *id).collect();
    let tbl: Vec<u16> = IXGBE_PCI_TBL.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, tbl);
    assert_eq!(INTEL_VENDOR_ID, 0x8086);
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "no id listed twice");
}
