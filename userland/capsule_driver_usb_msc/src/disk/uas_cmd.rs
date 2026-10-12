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
//! One SCSI command as nonos_uas sees it (its ScsiCommand): the CDB, which
//! way its data moves and how much, and the status and sense it ended with.

use nonos_uas::{DataDir, ScsiCommand};

use super::types::Data;
use crate::state::{State, SENSE_KEPT};

const REQUEST_SENSE: u8 = 0x03;

pub(super) struct Cmd {
    pub cdb: ([u8; 16], u8),
    pub dir: DataDir,
    pub len: u32,
    /// The status, the sense and its length, once the device said.
    pub ended: Option<(u8, [u8; SENSE_KEPT], usize)>,
}

impl ScsiCommand for Cmd {
    fn cdb(&self) -> &[u8] {
        &self.cdb.0[..self.cdb.1 as usize]
    }

    fn dir(&self) -> DataDir {
        self.dir
    }

    fn length(&self) -> u32 {
        self.len
    }

    fn finished(&mut self, status: u8, sense: &[u8]) {
        let mut kept = [0u8; SENSE_KEPT];
        let n = sense.len().min(SENSE_KEPT);
        kept[..n].copy_from_slice(&sense[..n]);
        self.ended = Some((status, kept, n));
    }
}

/// REQUEST SENSE answered from the sense the last CHECK CONDITION brought,
/// when there is some: the SCSI status and the residue.
pub(super) fn kept_sense(
    state: &mut State,
    cdb: ([u8; 16], u8),
    data: &mut Data,
) -> Option<(u8, u32)> {
    let Data::In(buf) = data else { return None };
    if cdb.0[0] != REQUEST_SENSE {
        return None;
    }
    let (sense, n) = state.take_sense()?;
    let n = n.min(buf.len());
    buf[..n].copy_from_slice(&sense[..n]);
    Some((0, (buf.len() - n) as u32))
}
