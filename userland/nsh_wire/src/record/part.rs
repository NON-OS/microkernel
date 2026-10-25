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

//! `RECORD`. A record arrives in parts, the last one flagged, so no single
//! frame has to hold a whole session's record.

use crate::limits::MAX_DATA;
use crate::WireError;

/// One part of a record: a flag that says whether it is the last, then 1
/// to `MAX_DATA - 1` bytes of it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecordPart<'a> {
    pub last: bool,
    pub bytes: &'a [u8],
}

impl<'a> RecordPart<'a> {
    pub fn parse(body: &'a [u8]) -> Result<Self, WireError> {
        let (flag, bytes) = body.split_first().ok_or(WireError::Short)?;
        let last = match flag {
            0 => false,
            1 => true,
            _ => return Err(WireError::Value),
        };
        match bytes.len() {
            0 => Err(WireError::Value),
            n if n > MAX_DATA - 1 => Err(WireError::Oversize),
            _ => Ok(RecordPart { last, bytes }),
        }
    }
}
