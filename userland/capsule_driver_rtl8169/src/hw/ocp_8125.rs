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

//! The MAC OCP words Linux rtl_hw_start_8125_common writes on every 8125,
//! in its order. Realtek gives them without names; Linux names three.

/// (register, bits cleared, bits set), as r8168_mac_ocp_modify takes them.
pub type OcpModify = (u32, u16, u16);

/// From "disable UPS" (0xD40A) up to the new TX descriptor format switch.
pub const BEFORE_FORMAT: &[OcpModify] = &[(0xD40A, 0x0010, 0x0000)];

/// After the 0xC140/0xC142 writes: 0xEB58 bit 0 clear is "disable new tx
/// descriptor format", without which the part reads the ring in a layout
/// this driver does not write.
pub const FORMAT: &[OcpModify] = &[
    (0xD3E2, 0x0FFF, 0x03A9),
    (0xD3E4, 0x00FF, 0x0000),
    (0xE860, 0x0000, 0x0080),
    (0xEB58, 0x0001, 0x0000),
];

/// After 0xE614 and 0xE63E, which differ on the 8125B.
pub const TAIL: &[OcpModify] = &[
    (0xC0B4, 0x0000, 0x000C),
    (0xEB6A, 0x00FF, 0x0033),
    (0xEB50, 0x03E0, 0x0040),
    (0xE056, 0x00F0, 0x0000),
    (0xE040, 0x1000, 0x0000),
    (0xEA1C, 0x0003, 0x0001),
    (0xEA1C, 0x0004, 0x0000),
    (0xE0C0, 0x4F0F, 0x4403),
    (0xE052, 0x0080, 0x0068),
    (0xD430, 0x0FFF, 0x047F),
    (0xEA1C, 0x0004, 0x0000),
];

/// 0xE614 and 0xE63E: the 8125B (VER_63) takes its own values.
pub fn by_version(ver: u8) -> [OcpModify; 2] {
    if ver == 63 {
        [(0xE614, 0x0700, 0x0200), (0xE63E, 0x0C30, 0x0000)]
    } else {
        [(0xE614, 0x0700, 0x0300), (0xE63E, 0x0C30, 0x0020)]
    }
}
