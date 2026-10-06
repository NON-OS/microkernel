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

/// The version a row of `rtl_chip_infos` gives when the XID only says "read
/// the extended id" (`RTL_GIGA_MAC_VER_EXTENDED`). No real version uses it.
pub const VER_EXTENDED: u8 = 0xFF;

/// One row of Linux `rtl_chip_infos`: the XID matches when `xid & mask == val`.
#[derive(Clone, Copy)]
pub struct XidEntry {
    pub mask: u16,
    pub val: u16,
    pub ver: u8,
    pub name: &'static str,
}

pub const fn e(mask: u16, val: u16, ver: u8, name: &'static str) -> XidEntry {
    XidEntry { mask, val, ver, name }
}
