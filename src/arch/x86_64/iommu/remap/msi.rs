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

//! The MSI message a device sends once its interrupt is remapped (VT-d 3.4,
//! 5.1.5.2): the address names an entry instead of a CPU, as Linux builds it
//! in intel/irq_remapping.c, fill_msi_msg.

/// Address and data to program into an MSI capability or an MSI-X table
/// entry. The upper address dword is zero.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MsiMessage {
    pub address: u32,
    pub data: u32,
}

const ADDRESS_BASE: u32 = 0xFEE0_0000;
/// Interrupt Format, bit 4: this is a remappable request.
const REMAPPABLE: u32 = 1 << 4;
/// SubHandle Valid, bit 3: the entry is handle plus the data's low sixteen
/// bits. The data is zero, so the entry is the handle itself.
const SUBHANDLE_VALID: u32 = 1 << 3;

/// The message that raises entry `index`. Handle bits 14:0 sit in address
/// bits 19:5 and handle bit 15 in address bit 2.
pub const fn remapped(index: u16) -> MsiMessage {
    let low = ((index as u32) & 0x7FFF) << 5;
    let high = (((index as u32) >> 15) & 1) << 2;
    MsiMessage { address: ADDRESS_BASE | REMAPPABLE | SUBHANDLE_VALID | low | high, data: 0 }
}

/// The entry index a remappable address names, `None` for a compatibility
/// format address.
pub const fn handle(address: u32) -> Option<u16> {
    if address & REMAPPABLE == 0 {
        return None;
    }
    Some((((address >> 5) & 0x7FFF) | (((address >> 2) & 1) << 15)) as u16)
}
