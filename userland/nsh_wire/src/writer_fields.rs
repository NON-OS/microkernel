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

//! The typed writes: little-endian integers and length-prefixed byte runs.

use crate::{WireError, Writer};

impl<'a> Writer<'a> {
    pub fn u8(&mut self, v: u8) -> Result<(), WireError> {
        self.put(&[v])
    }

    pub fn u16(&mut self, v: u16) -> Result<(), WireError> {
        self.put(&v.to_le_bytes())
    }

    pub fn u32(&mut self, v: u32) -> Result<(), WireError> {
        self.put(&v.to_le_bytes())
    }

    pub fn u64(&mut self, v: u64) -> Result<(), WireError> {
        self.put(&v.to_le_bytes())
    }

    /// A u8 length and the bytes. Over 255 is refused, never cut.
    pub fn bytes8(&mut self, bytes: &[u8]) -> Result<(), WireError> {
        let n = u8::try_from(bytes.len()).map_err(|_| WireError::Oversize)?;
        self.u8(n)?;
        self.put(bytes)
    }

    /// A u16 length and the bytes. Over 65535 is refused, never cut.
    pub fn bytes16(&mut self, bytes: &[u8]) -> Result<(), WireError> {
        let n = u16::try_from(bytes.len()).map_err(|_| WireError::Oversize)?;
        self.u16(n)?;
        self.put(bytes)
    }

    /// A u32 length and the bytes. Over u32::MAX is refused, never cut.
    pub fn bytes32(&mut self, bytes: &[u8]) -> Result<(), WireError> {
        let n = u32::try_from(bytes.len()).map_err(|_| WireError::Oversize)?;
        self.u32(n)?;
        self.put(bytes)
    }
}
