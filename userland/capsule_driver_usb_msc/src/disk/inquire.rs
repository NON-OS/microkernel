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

//! The INQUIRY that opens a probe. Linux's SCSI scan always opens with it,
//! and some USB storage firmware answers nothing else correctly until it
//! has seen one. Its data is not needed: a failed status is ignored, and a
//! transport that keeps failing ends the probe.

use nonos_libc::Deadline;

use super::bot::command;
use super::recover::reset_recovery;
use super::types::{Data, Disk};
use super::unit_ready::Pause;
use crate::protocol::E_IO;
use crate::scsi::{inquiry, INQUIRY_DATA_LEN};
use crate::state::State;

/// Tries, recovering the pipes between them: a stick fresh off its reset
/// can miss the first command.
const INQUIRY_TRIES: u32 = 5;

pub(super) fn inquire(
    disk: &Disk,
    state: &mut State,
    deadline: &Deadline,
    pause: Pause,
) -> Result<(), i32> {
    let mut data = [0u8; INQUIRY_DATA_LEN];
    let mut tries = 0;
    while command(disk, state, inquiry(), Data::In(&mut data)).is_err() {
        tries += 1;
        if tries == INQUIRY_TRIES || deadline.expired() {
            return Err(E_IO);
        }
        let _ = reset_recovery(disk);
        pause(state);
    }
    Ok(())
}
