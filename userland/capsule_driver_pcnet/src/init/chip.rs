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

//! Which part answered: CSR88 and CSR89 hold the chip id, AMD's
//! manufacturer code in the low twelve bits and the part number above.

use crate::constants::csr::{CHIP_ID_AMD, CSR88, CSR89};
use crate::regs::Regs;

pub fn identify(regs: &Regs) -> Result<u16, &'static str> {
    let id = regs.csr(CSR88) as u32 | (regs.csr(CSR89) as u32) << 16;
    part_number(id).ok_or("chip id is not an AMD PCnet")
}

/// The part number, for any id that carries AMD's code. Every PCnet-PCI part
/// takes SWSTYLE 2 and the init block the same way, so none is refused by
/// number: 0x2621 is the 79C970A, 0x2625 the 79C973.
pub const fn part_number(id: u32) -> Option<u16> {
    if id & 0xFFF != CHIP_ID_AMD {
        return None;
    }
    Some((id >> 12) as u16)
}
