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

//! The Realtek USB controllers this capsule patches: what each ROM
//! answers, its project, its file layout and its pinned files. RTL8822CU
//! is the 8822C row; its ROM answers HCI revision 000C where RTL8822B's
//! answers 000B.

use super::chip::Chip;
use super::pins::*;
use super::pins_config::*;
use super::pins_rtbt::*;
use super::row::{rtbt, usb};

pub const CHIPS: &[Chip] = &[
    usb("RTL8723B", (0x8723, 0x000b, 0x06), 1, &RTL8723B_FW, &RTL8723BS_CONFIG, false),
    usb("RTL8723D", (0x8723, 0x000d, 0x08), 9, &RTL8723D_FW, &RTL8821C_CONFIG, true),
    usb("RTL8821A", (0x8821, 0x000a, 0x06), 2, &RTL8821A_FW, &RTL8821C_CONFIG, false),
    usb("RTL8821C", (0x8821, 0x000c, 0x08), 10, &RTL8821C_FW, &RTL8821C_CONFIG, false),
    usb("RTL8761A", (0x8761, 0x000a, 0x06), 3, &RTL8761A_FW, &RTL8761BU_CONFIG, false),
    usb("RTL8761BU", (0x8761, 0x000b, 0x0a), 14, &RTL8761BU_FW, &RTL8761BU_CONFIG, false),
    usb("RTL8822B", (0x8822, 0x000b, 0x07), 8, &RTL8822B_FW, &RTL8822B_CONFIG, true),
    rtbt("RTL8822CU", (0x8822, 0x000c, 0x0a), 13, &RTL8822CU_FW),
    rtbt("RTL8852AU", (0x8852, 0x000a, 0x0b), 18, &RTL8852AU_FW),
    rtbt("RTL8852BU", (0x8852, 0x000b, 0x0b), 20, &RTL8852BU_FW),
    rtbt("RTL8852CU", (0x8852, 0x000c, 0x0c), 25, &RTL8852CU_FW),
    rtbt("RTL8851BU", (0x8851, 0x000b, 0x0c), 36, &RTL8851BU_FW),
    rtbt("RTL8852BTU", (0x8852, 0x0087, 0x0c), 47, &RTL8852BTU_FW),
];
