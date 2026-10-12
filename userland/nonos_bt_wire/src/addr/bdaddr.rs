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

//! A device address (Vol 6, Part B, 1.3). It travels as six octets, least
//! significant first (Vol 4, Part E, 5.2), and is held here in that order.
//! A random address's two most significant bits say which kind it is
//! (Vol 6, Part B, 1.3.2): 0b11 static, 0b01 resolvable private, 0b00
//! non-resolvable private, with 0b10 reserved.

use crate::error::WireError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BdAddr(pub [u8; 6]);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RandomKind {
    Static,
    Resolvable,
    NonResolvable,
}

impl BdAddr {
    /// What kind of random address this is, if read as one.
    pub fn random_kind(&self) -> Result<RandomKind, WireError> {
        match self.0[5] >> 6 {
            0b11 => Ok(RandomKind::Static),
            0b01 => Ok(RandomKind::Resolvable),
            0b00 => Ok(RandomKind::NonResolvable),
            _ => Err(WireError::Reserved),
        }
    }
}
