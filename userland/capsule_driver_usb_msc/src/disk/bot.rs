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
//!
//! Two things real devices do that the Bulk-Only specification does not
//! allow are taken as Linux's usb_stor_Bulk_transport takes them. Some end
//! a data phase with a zero-length packet it did not need, which reaches
//! the CSW read as an empty CSW; the CSW is read again. Some skip the data
//! phase of a command they fail and send the CSW at once, which reaches the
//! data read as 13 short bytes; those bytes are the CSW. Before this the
//! first failed every command with a transport error, and the second waited
//! out the bulk timeout for a CSW already sent, then reset the device.

use super::data_phase::data_phase;
use super::recover::{clear_halt, reset_recovery};
use super::types::{Data, Disk};
use crate::bot::{is_csw, parse, CommandBlockWrapper, CBW_FLAG_IN, CBW_FLAG_OUT};
use crate::protocol::{CBW_LEN, CSW_LEN, E_IO};
use crate::state::State;
use crate::xhci::{bulk_in, bulk_out, E_PIPE};

/// Run `cdb` with `data`. Returns the SCSI status (0 good, 1 CHECK
/// CONDITION) and the residue (`State::residue`). A transport that lost its
/// phase is reset and `E_IO` returned.
pub fn command(
    disk: &Disk,
    state: &mut State,
    cdb: ([u8; 16], u8),
    mut data: Data,
) -> Result<(u8, u32), i32> {
    let flags = if matches!(data, Data::In(_)) { CBW_FLAG_IN } else { CBW_FLAG_OUT };
    let data_len = data.len() as u32;
    let tag = state.begin_command(data_len);
    let lun = disk.lun;
    let cbw = CommandBlockWrapper { tag, data_len, flags, lun, cdb_len: cdb.1, cdb: cdb.0 };
    let mut raw = [0u8; CBW_LEN];
    cbw.write(&mut raw);
    if bulk_out(disk.xhci, disk.slot, &raw) != Ok(CBW_LEN) {
        let _ = reset_recovery(disk);
        return Err(E_IO);
    }
    let mut moved = match data_phase(disk, &mut data) {
        Ok(m) => m,
        Err(e) => {
            let _ = reset_recovery(disk);
            return Err(e);
        }
    };
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
        _ => Err(E_IO),
    };
    if status.is_err() {
        let _ = reset_recovery(disk);
    }
    status.map_err(|_| E_IO)
}

/// The CSW, read again after an empty packet and after a stall cleared.
fn read_csw(disk: &Disk, csw: &mut [u8; CSW_LEN]) -> Result<usize, i32> {
    let mut got = bulk_in(disk.xhci, disk.slot, csw);
    if got == Ok(0) {
        got = bulk_in(disk.xhci, disk.slot, csw);
    }
    if got == Err(E_PIPE) {
        clear_halt(disk, true)?;
        got = bulk_in(disk.xhci, disk.slot, csw);
    }
    got
}

/// The CSW a device sent in place of a data-in phase it skipped: exactly 13
/// short bytes that carry the CSW signature. Linux checks the same two
/// things; the tag is checked with every other CSW's.
fn skipped_data_phase(data: &Data, moved: usize) -> Option<[u8; CSW_LEN]> {
    let Data::In(buf) = data else { return None };
    if moved != CSW_LEN || buf.len() <= CSW_LEN || !is_csw(&buf[..CSW_LEN]) {
        return None;
    }
    buf[..CSW_LEN].try_into().ok()
}
