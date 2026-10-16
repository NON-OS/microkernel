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

//! What a CHECK CONDITION left: the sense, asked at once, which also clears
//! it.

use super::bot::command;
use super::types::{Data, Disk};
use crate::protocol::E_IO;
use crate::scsi::{parse_sense, request_sense, Sense};
use crate::state::{Said, State};

const SENSE_LEN: u8 = 18;
pub(super) const UNIT_ATTENTION: u8 = 0x06;
/// ASC NOT READY TO READY CHANGE, MEDIUM MAY HAVE CHANGED.
const MEDIUM_CHANGED: u8 = 0x28;
/// ASC MEDIUM NOT PRESENT: a card reader's slot with no card in it.
pub(super) const MEDIUM_NOT_PRESENT: u8 = 0x3A;

/// The sense of the last CHECK CONDITION, or None when the device answered
/// with none that reads; asking also clears it. An error is the transport's,
/// and counts as one (scan/pace.rs): a sense lost on the bus was taken for
/// a unit not yet ready, and never ended the asking. What it says of the
/// medium is kept in `state` until the scanner asks (`Said`): a UNIT
/// ATTENTION but no medium, or a sense that does not read, may be a change,
/// as may one lost on the bus (recover.rs `lost`).
pub(super) fn sense(disk: &Disk, state: &mut State) -> Result<Option<Sense>, i32> {
    let mut raw = [0u8; SENSE_LEN as usize];
    let (status, _) = command(disk, state, request_sense(SENSE_LEN), Data::In(&mut raw))?;
    let said = if status == 0 { parse_sense(&raw) } else { None };
    state.heard(match said.map(|s| (s.sense_key, s.asc)) {
        Some((UNIT_ATTENTION, MEDIUM_CHANGED)) => Said::Changed,
        Some((UNIT_ATTENTION, MEDIUM_NOT_PRESENT)) => Said::Nothing,
        Some((UNIT_ATTENTION, _)) | None => Said::Attention,
        Some(_) => Said::Nothing,
    });
    Ok(said)
}

/// A good status with every byte moved, or `E_IO` with the condition cleared.
pub(super) fn whole(disk: &Disk, state: &mut State, outcome: (u8, u32)) -> Result<(), i32> {
    match outcome {
        (0, 0) => Ok(()),
        (0, _) => Err(E_IO),
        _ => sense(disk, state).and(Err(E_IO)),
    }
}
