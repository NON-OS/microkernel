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

use super::entry::VER_EXTENDED;
use super::table_fast::FAST;
use super::table_giga::GIGA;
use super::Chip;

/// Linux rtl_init_one: `xid = (RTL_R32(tp, TxConfig) >> 20) & 0xfcf`.
pub fn xid_of(txconfig: u32) -> u32 {
    (txconfig >> 20) & 0xfcf
}

#[derive(Debug, PartialEq, Eq)]
pub enum Lookup {
    Known(Chip),
    /// The id lives in TX_CONFIG_V2; see `lookup_extended`.
    Extended,
    Unknown,
}

/// Linux rtl8169_get_chip_version: the first matching row, then the two
/// 10/100 boards that share a gigabit XID.
pub fn lookup(xid: u32, gmii: bool) -> Lookup {
    for row in GIGA.iter().chain(FAST.iter()) {
        if xid & row.mask as u32 != row.val as u32 {
            continue;
        }
        if row.ver == VER_EXTENDED {
            return Lookup::Extended;
        }
        let chip = match (row.ver, gmii) {
            (42, false) => Chip::new(43, "RTL8106eus", xid),
            (46, false) => Chip::new(48, "RTL8107e", xid),
            _ => Chip::new(row.ver, row.name, xid),
        };
        return Lookup::Known(chip);
    }
    Lookup::Unknown
}
