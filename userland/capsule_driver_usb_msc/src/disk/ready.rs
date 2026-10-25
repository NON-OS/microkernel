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

//! Bringing a logical unit up, one TEST UNIT READY at a time: the caller
//! (scan/ask.rs) keeps how long it may go on and pauses between them, so
//! the server answers requests while a unit spins up.

use super::sense::{sense, MEDIUM_NOT_PRESENT, UNIT_ATTENTION};
use super::transport::command;
use super::types::{Data, Disk};
use crate::scsi::{start_unit, test_unit_ready};
use crate::state::State;

const NOT_READY: u8 = 0x02;

/// What one TEST UNIT READY found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unit {
    Ready,
    /// No medium (ASC 3Ah, SPC, as recalled): said at once, under NOT
    /// READY or UNIT ATTENTION. It was waited out for thirty seconds on
    /// a reader's last slot, and an empty single-slot reader held the
    /// server that long every few seconds.
    NoMedium,
    /// Any other UNIT ATTENTION: a power-on, a reset, or a medium change,
    /// which a device says once (SAM, as recalled) and is asked past;
    /// `State` keeps it (`Said`).
    Attention,
    /// Spinning up, or a sense that did not read.
    NotYet,
}

/// One TEST UNIT READY, and the sense of a failed one. A unit that waits to
/// be told to spin up is sent one START UNIT (SBC START STOP UNIT, as
/// recalled), the first time it says NOT READY; `started` remembers. A transport that
/// fails under the TEST UNIT READY or its sense is an error. A START UNIT
/// that fails has its sense taken too, as any command's: a 28h the unit
/// failed it for is said only then, and unread it was lost, so a held
/// medium swapped under it was bound again rather than ended.
pub fn test_ready(disk: &Disk, state: &mut State, started: &mut bool) -> Result<Unit, i32> {
    if command(disk, state, test_unit_ready(), Data::None)?.0 == 0 {
        return Ok(Unit::Ready);
    }
    let Some(said) = sense(disk, state)? else { return Ok(Unit::NotYet) };
    Ok(match (said.sense_key, said.asc) {
        (NOT_READY | UNIT_ATTENTION, MEDIUM_NOT_PRESENT) => Unit::NoMedium,
        (UNIT_ATTENTION, _) => Unit::Attention,
        (NOT_READY, _) if !*started => {
            *started = true;
            let started_unit = command(disk, state, start_unit(), Data::None);
            if started_unit.is_ok_and(|(status, _)| status != 0) {
                let _ = sense(disk, state);
            }
            Unit::NotYet
        }
        _ => Unit::NotYet,
    })
}
