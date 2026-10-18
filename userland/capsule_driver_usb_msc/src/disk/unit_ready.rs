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

//! Waiting for a logical unit to be ready. The wait was 600 tries of 50 ms,
//! and each try is a whole BOT command with its own transfer timeouts, so a
//! stick that timed out every try held the probe far past the thirty
//! seconds meant; it is now a deadline. Between tries the caller's `pause`
//! runs, where the server answers its inbox.

use nonos_libc::Deadline;

use super::bot::command;
use super::inquire::inquire;
use super::scsi_io::sense;
use super::types::{Data, Disk};
use crate::protocol::{E_IO, E_NOMEDIUM};
use crate::scsi::{start_unit, test_unit_ready};
use crate::state::State;

const NOT_READY: u8 = 0x02;
/// ASC MEDIUM NOT PRESENT: a card reader's slot with no card in it.
const MEDIUM_NOT_PRESENT: u8 = 0x3A;
/// How long a medium is given to come ready: a USB-SATA bridge spinning up
/// a disk, or a slow stick, can take this long.
const READY_TIMEOUT_MS: u64 = 30_000;

/// What runs between readiness tries.
pub type Pause<'a> = &'a mut dyn FnMut(&mut State);

/// INQUIRY first (`inquire`), then TEST UNIT READY, with one START UNIT for
/// a medium that waits to be told to spin up, as Linux's sd_spinup_disk
/// sends.
///
/// A unit that says it has no medium is given up at once, as sd_spinup_disk
/// does. Only units before the last were, and a card reader with one slot
/// and no card held the probe the whole wait.
pub fn unit_ready(disk: &Disk, state: &mut State, pause: Pause) -> Result<(), i32> {
    let deadline = Deadline::after_ms(READY_TIMEOUT_MS);
    inquire(disk, state, &deadline, pause)?;
    let mut started = false;
    while !deadline.expired() {
        // A transport error while the medium spins up is one more not-ready
        // answer: `command` has recovered the pipes, and the next try goes on.
        if let Ok((status, _)) = command(disk, state, test_unit_ready(), Data::None) {
            if status == 0 {
                return Ok(());
            }
            let said = sense(disk, state);
            if said.is_ok_and(|s| s.asc == MEDIUM_NOT_PRESENT) {
                return Err(E_NOMEDIUM);
            }
            if said.is_ok_and(|s| s.sense_key == NOT_READY) && !started {
                started = true;
                let _ = command(disk, state, start_unit(), Data::None);
            }
        }
        pause(state);
    }
    Err(E_IO)
}
