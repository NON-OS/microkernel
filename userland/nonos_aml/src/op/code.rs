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

//! The opcode type.

/// An opcode as it is encoded: a single byte for the primary page, 0x5Bxx for
/// the extended page behind ExtOpPrefix, and 0xF0xx for the arena's own node
/// kinds that have no opcode of their own (section 20.3 never uses 0xF0).
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Op(pub u16);

impl Op {
    /// ExtOpPrefix, section 20.2.3.
    pub const EXT_PREFIX: u8 = 0x5B;

    pub const fn ext(byte: u8) -> Op {
        Op(0x5B00 | byte as u16)
    }

    pub const fn is_ext(self) -> bool {
        self.0 & 0xFF00 == 0x5B00
    }

    /// LocalObj, 0x60-0x67: the slot number if this is one.
    pub const fn local(self) -> Option<u8> {
        match self.0 {
            0x60..=0x67 => Some((self.0 - 0x60) as u8),
            _ => None,
        }
    }

    /// ArgObj, 0x68-0x6E: the slot number if this is one.
    pub const fn arg(self) -> Option<u8> {
        match self.0 {
            0x68..=0x6E => Some((self.0 - 0x68) as u8),
            _ => None,
        }
    }
}
