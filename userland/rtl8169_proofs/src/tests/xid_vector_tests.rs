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

//! One TxConfig per Linux rtl_chip_infos row family, and the version and
//! name Linux gives it (r8169_main.c).

use super::xid_tests::ver;

#[test]
fn each_revision_maps_to_its_linux_mac_version() {
    let vectors: &[(u32, u8, &str)] = &[
        (0x6c9, 80, "RTL8127A"),
        (0x64a, 70, "RTL8126A"),
        (0x681, 66, "RTL8125BP"),
        (0x708, 65, "RTL8125CP"),
        (0x688, 64, "RTL8125D"),
        (0x68a, 64, "RTL8125K"),
        (0x641, 63, "RTL8125B"),
        (0x609, 61, "RTL8125A"),
        (0x54a, 52, "RTL8168fp/RTL8117"),
        (0x502, 51, "RTL8168ep/8111ep"),
        (0x541, 46, "RTL8168h/8111h"),
        (0x6c0, 46, "RTL8168M"),
        (0x5c8, 44, "RTL8411b"),
        (0x509, 42, "RTL8168gu/8111gu"),
        (0x4c0, 40, "RTL8168g/8111g"),
        (0x48c, 38, "RTL8411"),
        (0x481, 36, "RTL8168f/8111f"),
        (0x480, 35, "RTL8168f/8111f"),
        (0x2c9, 34, "RTL8168evl/8111evl"),
        (0x2c1, 32, "RTL8168e/8111e"),
        (0x2c2, 33, "RTL8168e/8111e"),
        (0x281, 25, "RTL8168d/8111d"),
        (0x282, 26, "RTL8168d/8111d"),
        (0x28a, 28, "RTL8168dp/8111dp"),
        (0x28b, 31, "RTL8168dp/8111dp"),
        (0x3c9, 23, "RTL8168cp/8111cp"),
        (0x3c8, 18, "RTL8168cp/8111cp"),
        (0x3cc, 24, "RTL8168cp/8111cp"),
        (0x3c0, 19, "RTL8168c/8111c"),
        (0x3c4, 22, "RTL8168c/8111c"),
        (0x381, 17, "RTL8168b/8111b"),
        (0x448, 39, "RTL8106e"),
        (0x409, 29, "RTL8105e"),
        (0x249, 8, "RTL8102e"),
        (0x240, 14, "RTL8401"),
        (0x340, 10, "RTL8101e/RTL8100e"),
        (0x980, 6, "RTL8169sc/8110sc"),
        (0x040, 3, "RTL8110s"),
        (0x008, 2, "RTL8169s"),
    ];
    for &(xid, want, name) in vectors {
        // Bits 19..0 of TxConfig are configuration, not identity.
        assert_eq!(ver(xid << 20 | 0x700, true), (want, name), "xid {xid:#x}");
    }
}
