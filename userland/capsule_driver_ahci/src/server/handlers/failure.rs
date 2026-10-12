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
//! and error registers only when the disk ended the command with an error
//! (ERR set), a timeout when it never finished, no device once the port is
//! wedged (out of service until a reboot, as with no disk served), and an
//! I/O error otherwise: an HBA error or a link change among them, since
//! PxTFD then says nothing about the command.

use crate::constants::regs::TFD_ERR;
use crate::engine::Port;
use crate::error::AhciError;
use crate::protocol::{ata_status, E_IO, E_NODEV, E_TIMEDOUT};

pub fn status(port: &Port, e: AhciError) -> i32 {
    match e {
        AhciError::Timeout => E_TIMEDOUT,
        AhciError::CommandFailed if port.last_tfd & TFD_ERR != 0 => ata_status(port.last_tfd),
        AhciError::Wedged => E_NODEV,
        _ => E_IO,
    }
}
