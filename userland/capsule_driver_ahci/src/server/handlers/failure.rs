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

//! What a failed SATA command answers its client: the disk's own status
//! and error registers when it ended the command with an error, a timeout
//! when it never finished, an I/O error otherwise.

use crate::engine::Port;
use crate::error::AhciError;
use crate::protocol::{ata_status, E_IO, E_TIMEDOUT};

pub fn status(port: &Port, e: AhciError) -> i32 {
    match e {
        AhciError::Timeout => E_TIMEDOUT,
        AhciError::CommandFailed => ata_status(port.last_tfd),
        _ => E_IO,
    }
}
