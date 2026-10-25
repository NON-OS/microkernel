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

//! The typed reads: little-endian integers and length-prefixed byte runs.

use crate::{Reader, WireError};

impl<'a> Reader<'a> {
    pub fn u8(&mut self) -> Result<u8, WireError> {
        Ok(self.array::<1>()?[0])
    }

    pub fn u16(&mut self) -> Result<u16, WireError> {
        Ok(u16::from_le_bytes(*self.array()?))
    }

    pub fn u32(&mut self) -> Result<u32, WireError> {
        Ok(u32::from_le_bytes(*self.array()?))
    }

    pub fn u64(&mut self) -> Result<u64, WireError> {
        Ok(u64::from_le_bytes(*self.array()?))
    }

    /// A u8 length, then that many bytes, the length within `min..=max`.
    pub fn bytes8(&mut self, min: usize, max: usize) -> Result<&'a [u8], WireError> {
        let n = self.u8()? as usize;
        self.sized(n, min, max)
    }

    /// A u16 length, then that many bytes, the length within `min..=max`.
    pub fn bytes16(&mut self, min: usize, max: usize) -> Result<&'a [u8], WireError> {
        let n = self.u16()? as usize;
        self.sized(n, min, max)
    }

    /// A u32 length, then that many bytes, the length within `min..=max`.
    pub fn bytes32(&mut self, min: usize, max: usize) -> Result<&'a [u8], WireError> {
        let n = usize::try_from(self.u32()?).map_err(|_| WireError::Oversize)?;
        self.sized(n, min, max)
    }

    /*
     * The length is judged before any byte it covers is touched, so a
     * length over the cap is Oversize whether or not the bytes follow.
     */
    fn sized(&mut self, n: usize, min: usize, max: usize) -> Result<&'a [u8], WireError> {
        match n {
            n if n > max => Err(WireError::Oversize),
            n if n < min => Err(WireError::Value),
            n => self.take(n),
        }
    }
}
