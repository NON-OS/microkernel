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

//! LE has two address types, public and random (Vol 6, Part B, 1.3), sent
//! as one octet, e.g. the Peer_Address_Type of LE Create Connection (Vol 4,
//! Part E, 7.8.12).

use crate::error::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AddrType {
    Public,
    Random,
}

impl AddrType {
    pub fn from_wire(v: u8) -> Result<Self, WireError> {
        match v {
            0x00 => Ok(Self::Public),
            0x01 => Ok(Self::Random),
            _ => Err(WireError::Reserved),
        }
    }

    /// The address type of an advertising report, which may also be 0x02
    /// or 0x03: a public or random identity address the controller resolved
    /// from a resolvable private one (Vol 4, Part E, 7.7.65.2).
    pub fn from_report(v: u8) -> Result<Self, WireError> {
        match v {
            0x00 | 0x02 => Ok(Self::Public),
            0x01 | 0x03 => Ok(Self::Random),
            _ => Err(WireError::Reserved),
        }
    }

    pub fn wire(self) -> u8 {
        match self {
            Self::Public => 0x00,
            Self::Random => 0x01,
        }
    }
}
