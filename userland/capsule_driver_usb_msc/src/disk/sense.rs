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

//! REQUEST SENSE after a CHECK CONDITION, read from the bytes the device
//! sent and no further.

use super::transport::command;
use super::types::{Data, Disk};
use crate::protocol::E_IO;
use crate::scsi::{parse_sense, request_sense, Sense};
use crate::state::State;

const SENSE_LEN: u8 = 18;

/// The sense of the last CHECK CONDITION; asking also clears it.
pub(super) fn sense(disk: &Disk, state: &mut State) -> Result<Sense, i32> {
    let mut raw = [0u8; SENSE_LEN as usize];
    // Only the bytes the device sent are sense: the rest of the buffer is
    // not its answer, and a short one must not read as a sense key of 0.
    match command(disk, state, request_sense(SENSE_LEN), Data::In(&mut raw))? {
        (0, residue) => {
            let sent = (SENSE_LEN as usize).saturating_sub(residue as usize);
            parse_sense(&raw[..sent]).ok_or(E_IO)
        }
        _ => Err(E_IO),
    }
}

/// The sense key of the last CHECK CONDITION; asking also clears it.
pub(super) fn sense_key(disk: &Disk, state: &mut State) -> Result<u8, i32> {
    sense(disk, state).map(|s| s.sense_key)
}
