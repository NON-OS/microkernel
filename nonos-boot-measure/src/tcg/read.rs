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

//! A cursor that never reads past its bytes. A short read says how many bytes
//! the field needed, so a reader that holds only a prefix can fetch exactly
//! that much more and try again.

use super::error::LogError;

pub(crate) enum Stop {
    /// The bytes end before the field; the field needs this many in all.
    Short(usize),
    Bad(LogError),
}

impl From<LogError> for Stop {
    fn from(e: LogError) -> Stop {
        Stop::Bad(e)
    }
}

pub(crate) struct Cursor<'a> {
    b: &'a [u8],
    at: usize,
}

impl<'a> Cursor<'a> {
    pub(crate) fn new(b: &'a [u8]) -> Cursor<'a> {
        Cursor { b, at: 0 }
    }

    pub(crate) fn at(&self) -> usize {
        self.at
    }

    pub(crate) fn take(&mut self, n: usize) -> Result<&'a [u8], Stop> {
        let end = self.at.checked_add(n).ok_or(Stop::Bad(LogError::Truncated))?;
        let s = self.b.get(self.at..end).ok_or(Stop::Short(end))?;
        self.at = end;
        Ok(s)
    }

    pub(crate) fn u8(&mut self) -> Result<u8, Stop> {
        self.take(1).map(|s| s[0])
    }

    pub(crate) fn u16(&mut self) -> Result<u16, Stop> {
        self.take(2).map(|s| u16::from_le_bytes([s[0], s[1]]))
    }

    pub(crate) fn u32(&mut self) -> Result<u32, Stop> {
        self.take(4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]))
    }
}

/// A short read, for a caller that holds the whole log, is a truncation.
pub(crate) fn whole<T>(r: Result<T, Stop>) -> Result<T, LogError> {
    r.map_err(|s| match s {
        Stop::Short(_) => LogError::Truncated,
        Stop::Bad(e) => e,
    })
}
