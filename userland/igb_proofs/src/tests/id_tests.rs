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
//! The device table against the igb table as observed in Linux igb_main.c
//! (unverified), entry for entry, by the part names given there.

use crate::facts::ids::{IGB_PARTS, INTEL_VENDOR_ID};

/// The table as observed in Linux igb_main.c, in its order (unverified).
const IGB_PCI_TBL: [(u16, &str); 35] = [
    (0x1F40, "I354_BACKPLANE_1GBPS"),
    (0x1F41, "I354_SGMII"),
    (0x1F45, "I354_BACKPLANE_2_5GBPS"),
    (0x1539, "I211_COPPER"),
    (0x1533, "I210_COPPER"),
    (0x1536, "I210_FIBER"),
    (0x1537, "I210_SERDES"),
    (0x1538, "I210_SGMII"),
    (0x157B, "I210_COPPER_FLASHLESS"),
    (0x157C, "I210_SERDES_FLASHLESS"),
    (0x1521, "I350_COPPER"),
    (0x1522, "I350_FIBER"),
    (0x1523, "I350_SERDES"),
    (0x1524, "I350_SGMII"),
    (0x150E, "82580_COPPER"),
    (0x150F, "82580_FIBER"),
    (0x1527, "82580_QUAD_FIBER"),
    (0x1510, "82580_SERDES"),
    (0x1511, "82580_SGMII"),
    (0x1516, "82580_COPPER_DUAL"),
    (0x0438, "DH89XXCC_SGMII"),
    (0x043A, "DH89XXCC_SERDES"),
    (0x043C, "DH89XXCC_BACKPLANE"),
    (0x0440, "DH89XXCC_SFP"),
    (0x10C9, "82576"),
    (0x150A, "82576_NS"),
    (0x1518, "82576_NS_SERDES"),
    (0x10E6, "82576_FIBER"),
    (0x10E7, "82576_SERDES"),
    (0x150D, "82576_SERDES_QUAD"),
    (0x1526, "82576_QUAD_COPPER_ET2"),
    (0x10E8, "82576_QUAD_COPPER"),
    (0x10A7, "82575EB_COPPER"),
    (0x10A9, "82575EB_FIBER_SERDES"),
    (0x10D6, "82575GB_QUAD_COPPER"),
];

#[test]
fn every_listed_id_is_bound_in_order_and_nothing_else() {
    let ids: Vec<u16> = IGB_PARTS.iter().map(|(id, ..)| *id).collect();
    let tbl: Vec<u16> = IGB_PCI_TBL.iter().map(|(id, _)| *id).collect();
    assert_eq!(ids, tbl);
    assert_eq!(INTEL_VENDOR_ID, 0x8086);
    let mut sorted = ids.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len(), "no id listed twice");
}
