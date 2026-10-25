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

//! The sense data of a CHECK CONDITION, fixed (0x70, 0x71) or descriptor
//! format (0x72, 0x73): its key, ASC and ASCQ.

use super::codes::CHECK_CONDITION;
use crate::ufs::error::UfsError;

pub fn parse(s: &[u8]) -> UfsError {
    let code = s.first().map(|b| b & 0x7F);
    match code {
        Some(0x70 | 0x71) if s.len() >= 14 => {
            UfsError::Sense { key: s[2] & 0xF, asc: s[12], ascq: s[13] }
        }
        Some(0x72 | 0x73) if s.len() >= 4 => {
            UfsError::Sense { key: s[1] & 0xF, asc: s[2], ascq: s[3] }
        }
        // A CHECK CONDITION with no sense the driver can read.
        _ => UfsError::Status(CHECK_CONDITION),
    }
}
