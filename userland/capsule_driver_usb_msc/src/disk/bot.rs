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

//! One BOT command: the CBW out, the data phase in pieces of one controller
//! page at most, and the CSW back (`csw`), checked against the command's tag.

use super::csw::{read_csw, skipped_data_phase};
use super::data_phase::data_phase;
use super::recover::lost;
use super::types::{Data, Disk};
use crate::bot::{parse, CommandBlockWrapper, CBW_FLAG_IN, CBW_FLAG_OUT};
use crate::protocol::{CBW_LEN, CSW_LEN, E_IO};
use crate::state::State;
use crate::xhci::bulk_out;

/// Run `cdb` with `data`: the SCSI status (0 good, 1 CHECK CONDITION) and
/// the residue (`State::residue`). A transport that lost its phase is reset
/// (`lost`).
pub fn command(
    disk: &Disk,
    state: &mut State,
    cdb: ([u8; 16], u8),
    mut data: Data,
) -> Result<(u8, u32), i32> {
    let flags = if matches!(data, Data::In(_)) { CBW_FLAG_IN } else { CBW_FLAG_OUT };
    let data_len = data.len() as u32;
    let tag = state.begin_command(data_len);
    let cbw =
        CommandBlockWrapper { tag, data_len, flags, lun: disk.lun, cdb_len: cdb.1, cdb: cdb.0 };
    let mut raw = [0u8; CBW_LEN];
    cbw.write(&mut raw);
    match bulk_out(disk.xhci, disk.slot, &raw) {
        Ok(CBW_LEN) => {}
        Ok(_) => return Err(lost(disk, state, E_IO)),
        Err(e) => return Err(lost(disk, state, e)),
    }
    let mut moved = data_phase(disk, &mut data).map_err(|e| lost(disk, state, e))?;
    let mut csw = [0u8; CSW_LEN];
    let got = match skipped_data_phase(&data, moved) {
        Some(early) => {
            (csw, moved) = (early, 0);
            Ok(CSW_LEN)
        }
        None => read_csw(disk, &mut csw),
    };
    let status = match got {
        Ok(CSW_LEN) => parse(&csw).and_then(|c| {
            let status = state.finish_command(c)?;
            Ok((status, state.residue(cdb.0[0], data_len, moved as u32, c)))
        }),
        other => other.and(Err(E_IO)),
    };
    status.map_err(|e| lost(disk, state, e))
}
