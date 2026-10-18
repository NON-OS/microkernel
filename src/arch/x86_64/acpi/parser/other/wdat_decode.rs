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

//! One WDAT instruction, 24 bytes: the action u8, the instruction flags
//! u8 (bits 1:0 the kind: read value, read countdown, write value, write
//! countdown; bit 7 preserve the register's other bits), two reserved
//! bytes, the register's GAS, the value u32 and the mask u32. Recalled
//! from Microsoft's published WDAT description, not read: unverified.

pub const ENTRY_LEN: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    ReadValue,
    ReadCountdown,
    WriteValue,
    WriteCountdown,
}

/// A register as its GAS gives it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Reg {
    pub space: u8,
    pub bit_width: u8,
    pub bit_offset: u8,
    pub access_size: u8,
    pub address: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Instruction {
    pub action: u8,
    pub kind: Kind,
    pub preserve: bool,
    pub reg: Reg,
    pub value: u32,
    pub mask: u32,
}

/// None for an unknown kind, or a register outside memory and I/O.
pub(super) fn instruction(e: &[u8]) -> Option<Instruction> {
    let word = |at: usize| u32::from_le_bytes([e[at], e[at + 1], e[at + 2], e[at + 3]]);
    let kind = match e[1] & 0x7F {
        0 => Kind::ReadValue,
        1 => Kind::ReadCountdown,
        2 => Kind::WriteValue,
        3 => Kind::WriteCountdown,
        _ => return None,
    };
    let address = u64::from(word(8)) | u64::from(word(12)) << 32;
    let reg = Reg { space: e[4], bit_width: e[5], bit_offset: e[6], access_size: e[7], address };
    if reg.space > 1 || reg.bit_offset >= 64 {
        return None;
    }
    let (value, mask) = (word(16), word(20));
    Some(Instruction { action: e[0], kind, preserve: e[1] & 0x80 != 0, reg, value, mask })
}
