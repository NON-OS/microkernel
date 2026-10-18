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

//! The ACL Packet_Boundary flag (Vol 4, Part E, 5.4.2).

/// The Packet_Boundary flag. A host sends the first fragment of an LE PDU
/// as `FirstNonFlushable`; a controller delivers it as `FirstFlushable`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boundary {
    FirstNonFlushable,
    Continuing,
    FirstFlushable,
    Complete,
}

impl Boundary {
    pub fn from_bits(pb: u16) -> Self {
        match pb & 0b11 {
            0b00 => Self::FirstNonFlushable,
            0b01 => Self::Continuing,
            0b10 => Self::FirstFlushable,
            _ => Self::Complete,
        }
    }

    pub fn bits(self) -> u16 {
        match self {
            Self::FirstNonFlushable => 0b00,
            Self::Continuing => 0b01,
            Self::FirstFlushable => 0b10,
            Self::Complete => 0b11,
        }
    }
}
