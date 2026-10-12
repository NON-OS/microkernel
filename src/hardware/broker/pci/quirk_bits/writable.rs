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

//! The vendor bits each kind of function may change.

use super::Ident;

const INTEL: u16 = 0x8086;
const AMD: u16 = 0x1022;
const ATI: u16 = 0x1002;

/// The bits at `offset` a driver of `id` may change, or `None` when the
/// register is not one of these.
pub fn writable(id: &Ident, offset: u32) -> Option<u16> {
    let audio = id.class == 0x04 && matches!(id.subclass, 0x01 | 0x03);
    let smbus = id.class == 0x0C && id.subclass == 0x05;
    let uhci = id.class == 0x0C && id.subclass == 0x03 && id.prog_if == 0x00;
    match (id.vendor, offset) {
        (INTEL, 0x40) if smbus => Some(0x0005),
        (_, 0xC0) if uhci => Some(0xAFFF),
        (INTEL, 0x44) if audio => Some(0x0007),
        (INTEL, 0x48) if audio => Some(0x0040),
        (INTEL, 0x78) if audio => Some(0x0800),
        (AMD | ATI, 0x42) if audio => Some(0x0007),
        _ if id.class == 0x02 && id.pcie_cap.is_some_and(|c| offset == c as u32 + 0x28) => {
            Some(0x001F)
        }
        _ => None,
    }
}

/// Whether `new` differs from `current` only in the bits `mask` allows.
pub fn only(mask: u16, new: u16, current: u16) -> bool {
    (new ^ current) & !mask == 0
}
