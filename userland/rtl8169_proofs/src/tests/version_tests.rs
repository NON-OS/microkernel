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

//! The version ranges against Linux r8169_main.c: rtl_is_8125,
//! rtl_is_8168evl_up, rtl_hw_initialize's 8168g arm, the rtl_hw_start_8169
//! cut-off, and the VER_18 boundary for a 64-bit DMA mask in rtl_init_one.

use crate::chip::MacVersion;

/// Every version Linux still lists in enum mac_version, up to VER_66.
const ALL: [u8; 45] = [
    2, 3, 4, 5, 6, 7, 8, 9, 10, 14, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 28, 29, 30, 31, 32, 33,
    34, 35, 36, 37, 38, 39, 40, 42, 43, 44, 46, 48, 51, 52, 61, 63, 64, 65, 66,
];

#[test]
fn each_predicate_holds_on_exactly_the_linux_range() {
    for v in ALL {
        let ver = MacVersion(v);
        assert_eq!(ver.is_8169(), v <= 6, "is_8169 VER_{v}");
        assert_eq!(ver.is_8125(), v >= 61, "is_8125 VER_{v}");
        assert_eq!(ver.is_8168evl_up(), (34..=52).contains(&v) && v != 39, "evl_up VER_{v}");
        assert_eq!(ver.is_8168g_up(), (40..=52).contains(&v), "8168g VER_{v}");
        assert_eq!(ver.dma32_only(), v < 18, "dma32 VER_{v}");
    }
}

#[test]
fn the_dma_boundary_falls_between_the_8168b_and_the_8168cp() {
    assert!(MacVersion(17).dma32_only(), "RTL8168b/8111b takes 32-bit addresses");
    assert!(MacVersion(10).dma32_only(), "RTL8101e");
    assert!(!MacVersion(18).dma32_only(), "RTL8168cp/8111cp takes 64-bit addresses");
    assert!(!MacVersion(63).dma32_only());
}
