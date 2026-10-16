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
//! One UAS command: the Command IU out on the command pipe,
//! then the status pipe read for what comes next (READ READY or WRITE
//! READY before the data moves on the data pipes, a Sense IU at the end),
//! nonos_uas's Exchange keeping the order. One command is in flight, its
//! tag not the last one's (State::uas_tag). A CHECK CONDITION's sense
//! comes with it and is kept for the REQUEST SENSE the driver asks next.

use nonos_uas::{command_iu, status_iu, Exchange, Step, COMMAND_IU_LEN};

use super::types::{Data, Disk};
use super::uas_cmd::{kept_sense, Cmd};
use super::uas_data::{dir_of, lane, move_data};
use crate::protocol::{E_INVAL, E_IO};
use crate::state::State;
use crate::xhci::{bulk_in_on, bulk_out_on};

/// A Sense IU's 16 bytes and its sense, in one high-speed packet.
const STATUS_MAX: usize = 512;

pub fn command(
    disk: &Disk,
    state: &mut State,
    cdb: ([u8; 16], u8),
    mut data: Data,
) -> Result<(u8, u32), i32> {
    let len = data.len();
    if let Some(answer) = kept_sense(state, cdb, &mut data) {
        return Ok(answer);
    }
    let dir = dir_of(&data);
    let mut cmd = Cmd { cdb, dir, len: len as u32, ended: None };
    let tag = state.uas_tag();
    let iu = command_iu(tag, disk.lun, &cmd).ok_or(E_INVAL)?;
    if bulk_out_on(disk.xhci, disk.slot, 0, &iu) != Ok(COMMAND_IU_LEN) {
        return Err(E_IO);
    }
    let (mut x, mut moved, mut buf) = (Exchange::new(tag, dir, disk.streams), 0, [0u8; STATUS_MAX]);
    let status_lane = lane(disk, 0, tag);
    let mut step = x.sent();
    loop {
        step = match step {
            Step::Status => {
                let n = bulk_in_on(disk.xhci, disk.slot, status_lane, &mut buf)?;
                let iu = status_iu(&buf[..n]).ok_or(E_IO)?;
                x.status(iu, &mut cmd)
            }
            Step::DataIn | Step::DataOut => {
                moved = move_data(disk, &mut data, tag)?;
                x.moved()
            }
            Step::Done => break,
            Step::Failed(_) => return Err(E_IO),
        };
    }
    let (status, sense, n) = cmd.ended.ok_or(E_IO)?;
    if status != 0 {
        state.keep_sense(&sense[..n]);
    }
    Ok((status, (len - moved) as u32))
}
