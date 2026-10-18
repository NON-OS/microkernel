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

//! The event that says whether a link is now encrypted.

use crate::cursor::Reader;
use crate::error::WireError;

/// Encryption_Enabled on an LE link: 0x00 off, 0x01 on with AES-CCM
/// (Vol 4, Part E, 7.7.8).
pub const ENCRYPTION_ON_AES_CCM: u8 = 0x01;

/// Encryption Change (Vol 4, Part E, 7.7.8).
pub struct EncryptionChange {
    pub status: u8,
    pub handle: u16,
    pub enabled: u8,
}

impl EncryptionChange {
    pub fn parse(params: &[u8]) -> Result<Self, WireError> {
        let mut r = Reader::new(params);
        let (status, handle, enabled) = (r.u8()?, r.u16()? & 0x0fff, r.u8()?);
        r.finish()?;
        Ok(Self { status, handle, enabled })
    }
}
