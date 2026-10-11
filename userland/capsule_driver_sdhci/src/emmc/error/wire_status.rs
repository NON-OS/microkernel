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

//! The status a failed request answers its client.

use super::{EmmcError, CARD_ERROR_BASE, HOST_ERROR_BASE};

pub const fn wire_status(e: EmmcError) -> i32 {
    match e {
        EmmcError::CmdTimeout(_) | EmmcError::NoCompletion(_) | EmmcError::CardStuck => -110,
        EmmcError::CmdError { cmd, err } | EmmcError::DataError { cmd, err } => {
            -(HOST_ERROR_BASE | ((cmd as i32 & 0x3f) << 16) | err as i32)
        }
        EmmcError::Status { cmd, status } => {
            let bit = if status == 0 { 0 } else { 31 - status.leading_zeros() as i32 };
            -(CARD_ERROR_BASE | ((cmd as i32 & 0x3f) << 16) | bit)
        }
        _ => -5,
    }
}
