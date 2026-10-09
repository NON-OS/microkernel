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

//! The 10/100 RTL810x rows, the PCI RTL8169/8110 rows and the extended-id
//! marker of Linux `rtl_chip_infos`, in its order, after the gigabit rows.

use super::entry::{e, XidEntry, VER_EXTENDED};

pub const FAST: &[XidEntry] = &[
    e(0x7c8, 0x448, 39, "RTL8106e"),
    e(0x7c8, 0x440, 37, "RTL8402"),
    e(0x7cf, 0x409, 29, "RTL8105e"),
    e(0x7c8, 0x408, 30, "RTL8105e"),
    e(0x7cf, 0x349, 8, "RTL8102e"),
    e(0x7cf, 0x249, 8, "RTL8102e"),
    e(0x7cf, 0x348, 7, "RTL8102e"),
    e(0x7cf, 0x248, 7, "RTL8102e"),
    e(0x7cf, 0x240, 14, "RTL8401"),
    e(0x7c8, 0x348, 9, "RTL8102e/RTL8103e"),
    e(0x7c8, 0x248, 9, "RTL8102e/RTL8103e"),
    e(0x7c8, 0x340, 10, "RTL8101e/RTL8100e"),
    e(0xfc8, 0x980, 6, "RTL8169sc/8110sc"),
    e(0xfc8, 0x180, 5, "RTL8169sc/8110sc"),
    e(0xfc8, 0x100, 4, "RTL8169sb/8110sb"),
    e(0xfc8, 0x040, 3, "RTL8110s"),
    e(0xfc8, 0x008, 2, "RTL8169s"),
    e(0x7cf, 0x7c8, VER_EXTENDED, ""),
];
