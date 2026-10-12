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

//! A bond record from its 48 octets. Every flag, type, size and reserved
//! octet is checked, as storage may hand back anything: a record this
//! version did not write is refused, not guessed at.

use super::layout::*;
use super::types::BondRecord;
use crate::error::BondError;

impl BondRecord {
    pub fn decode(bytes: &[u8]) -> Result<Self, BondError> {
        let b: &[u8; RECORD_LEN] = bytes.try_into().map_err(|_| BondError::Length)?;
        if b[0] != VERSION {
            return Err(BondError::Version);
        }
        let flags = b[1];
        if flags & !KNOWN_FLAGS != 0 || flags & SECURE_CONNECTIONS == 0 {
            return Err(BondError::Invalid);
        }
        if b[RESERVED..].iter().any(|&x| x != 0) {
            return Err(BondError::Invalid);
        }
        let irk: [u8; 16] = b[IRK..KEY_SIZE].try_into().map_err(|_| BondError::Length)?;
        let has_irk = flags & HAS_IRK != 0;
        if !has_irk && irk != [0; 16] {
            return Err(BondError::Invalid);
        }
        let record = Self {
            authenticated: flags & AUTHENTICATED != 0,
            identity_type: b[2],
            identity: b[ADDR..LTK].try_into().map_err(|_| BondError::Length)?,
            ltk: b[LTK..IRK].try_into().map_err(|_| BondError::Length)?,
            irk: has_irk.then_some(irk),
            key_size: b[KEY_SIZE],
        };
        record.check()?;
        Ok(record)
    }
}
