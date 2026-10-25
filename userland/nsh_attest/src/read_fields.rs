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

//! The typed reads: integers, flags and length-prefixed fields.

use crate::read::Be;
use crate::FormatFault;

impl<'a> Be<'a> {
    pub(crate) fn u8(&mut self) -> Result<u8, FormatFault> {
        Ok(self.array::<1>()?[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16, FormatFault> {
        Ok(u16::from_be_bytes(*self.array()?))
    }

    pub(crate) fn u32(&mut self) -> Result<u32, FormatFault> {
        Ok(u32::from_be_bytes(*self.array()?))
    }

    pub(crate) fn u64(&mut self) -> Result<u64, FormatFault> {
        Ok(u64::from_be_bytes(*self.array()?))
    }

    /// A flag byte: 0 or 1, nothing else.
    pub(crate) fn flag(&mut self) -> Result<bool, FormatFault> {
        match self.u8()? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(FormatFault::Value),
        }
    }

    /// A u16 or u32 length (`wide`), then that many bytes, within min..=max.
    pub(crate) fn sized(
        &mut self,
        wide: bool,
        min: usize,
        max: usize,
    ) -> Result<&'a [u8], FormatFault> {
        let n = if wide { self.u32()? as usize } else { self.u16()? as usize };
        match n {
            n if n > max => Err(FormatFault::Oversize),
            n if n < min => Err(FormatFault::Value),
            n => self.take(n),
        }
    }
}
