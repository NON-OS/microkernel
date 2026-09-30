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

//! One BOT command: the CBW out, the data phase in pieces of at most one
//! controller page, and the CSW back, checked against the command's tag.

use super::data_phase::data_phase;
use super::recover::{clear_halt, reset_recovery};
use super::types::{Data, Disk};
use crate::bot::{parse, CommandBlockWrapper, CBW_FLAG_IN, CBW_FLAG_OUT};
use crate::protocol::{CBW_LEN, CSW_LEN, E_IO};
use crate::state::State;
use crate::xhci::{bulk_in, bulk_out, E_PIPE};

/// Run `cdb` with `data`. Returns the SCSI status (0 good, 1 CHECK
/// CONDITION) and the CSW residue. A transport that lost its phase is reset
/// and `E_IO` returned.
pub fn command(
    disk: &Disk,
    state: &mut State,
    cdb: ([u8; 16], u8),
    data: Data,
) -> Result<(u8, u32), i32> {
    let flags = if matches!(data, Data::In(_)) { CBW_FLAG_IN } else { CBW_FLAG_OUT };
    let data_len = data.len() as u32;
    let tag = state.begin_command(data_len);
    let cbw = CommandBlockWrapper { tag, data_len, flags, lun: 0, cdb_len: cdb.1, cdb: cdb.0 };
    let mut raw = [0u8; CBW_LEN];
    cbw.write(&mut raw);
    if bulk_out(disk.xhci, disk.slot, &raw) != Ok(CBW_LEN) {
        let _ = reset_recovery(disk);
        return Err(E_IO);
    }
    if let Err(e) = data_phase(disk, data) {
        let _ = reset_recovery(disk);
        return Err(e);
    }
    let mut csw = [0u8; CSW_LEN];
    let mut got = bulk_in(disk.xhci, disk.slot, &mut csw);
    if got == Err(E_PIPE) {
        clear_halt(disk, true)?;
        got = bulk_in(disk.xhci, disk.slot, &mut csw);
    }
    let status = match got {
        Ok(CSW_LEN) => parse(&csw).and_then(|c| Ok((state.finish_command(c)?, c.residue))),
        _ => Err(E_IO),
    };
    if status.is_err() {
        let _ = reset_recovery(disk);
    }
    status.map_err(|_| E_IO)
}
