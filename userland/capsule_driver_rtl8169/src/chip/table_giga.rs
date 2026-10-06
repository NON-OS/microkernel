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

//! The 8127, 8126, 8125, 8117 and 8168 rows of Linux `rtl_chip_infos`
//! (r8169_main.c), in its order. The first row that matches names the chip,
//! and some XIDs match more than one row, so the order is part of the table.

use super::entry::{e, XidEntry};

pub const GIGA: &[XidEntry] = &[
    e(0x7cf, 0x6c9, 80, "RTL8127A"),
    e(0x7cf, 0x64a, 70, "RTL8126A"),
    e(0x7cf, 0x649, 70, "RTL8126A"),
    e(0x7cf, 0x681, 66, "RTL8125BP"),
    e(0x7cf, 0x708, 65, "RTL8125CP"),
    e(0x7cf, 0x68b, 64, "RTL9151A"),
    e(0x7cf, 0x68a, 64, "RTL8125K"),
    e(0x7cf, 0x689, 64, "RTL8125D"),
    e(0x7cf, 0x688, 64, "RTL8125D"),
    e(0x7cf, 0x641, 63, "RTL8125B"),
    e(0x7cf, 0x609, 61, "RTL8125A"),
    e(0x7cf, 0x54b, 52, "RTL8168fp/RTL8117"),
    e(0x7cf, 0x54a, 52, "RTL8168fp/RTL8117"),
    e(0x7cf, 0x502, 51, "RTL8168ep/8111ep"),
    e(0x7cf, 0x541, 46, "RTL8168h/8111h"),
    e(0x7cf, 0x6c0, 46, "RTL8168M"),
    e(0x7cf, 0x5c8, 44, "RTL8411b"),
    e(0x7cf, 0x509, 42, "RTL8168gu/8111gu"),
    e(0x7cf, 0x4c0, 40, "RTL8168g/8111g"),
    e(0x7c8, 0x488, 38, "RTL8411"),
    e(0x7cf, 0x481, 36, "RTL8168f/8111f"),
    e(0x7cf, 0x480, 35, "RTL8168f/8111f"),
    e(0x7c8, 0x2c8, 34, "RTL8168evl/8111evl"),
    e(0x7cf, 0x2c1, 32, "RTL8168e/8111e"),
    e(0x7c8, 0x2c0, 33, "RTL8168e/8111e"),
    e(0x7cf, 0x281, 25, "RTL8168d/8111d"),
    e(0x7c8, 0x280, 26, "RTL8168d/8111d"),
    e(0x7cf, 0x28a, 28, "RTL8168dp/8111dp"),
    e(0x7cf, 0x28b, 31, "RTL8168dp/8111dp"),
    e(0x7cf, 0x3c9, 23, "RTL8168cp/8111cp"),
    e(0x7cf, 0x3c8, 18, "RTL8168cp/8111cp"),
    e(0x7c8, 0x3c8, 24, "RTL8168cp/8111cp"),
    e(0x7cf, 0x3c0, 19, "RTL8168c/8111c"),
    e(0x7cf, 0x3c2, 20, "RTL8168c/8111c"),
    e(0x7cf, 0x3c3, 21, "RTL8168c/8111c"),
    e(0x7c8, 0x3c0, 22, "RTL8168c/8111c"),
    e(0x7c8, 0x380, 17, "RTL8168b/8111b"),
];
