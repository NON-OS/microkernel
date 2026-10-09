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

use super::Chip;

/// TX_CONFIG_V2, where a chip whose XID is the extended marker keeps its id.
pub const REG_TX_CONFIG_V2: usize = 0x60b0;

/// Linux rtl8169_get_extended_chip_version: one row today, the RTL9151AS.
pub fn lookup_extended(xid2: u32) -> Option<Chip> {
    if xid2 & 0x7fff_ffff == 0 {
        let mut chip = Chip::new(64, "RTL9151AS", xid2);
        chip.extended = true;
        return Some(chip);
    }
    None
}
