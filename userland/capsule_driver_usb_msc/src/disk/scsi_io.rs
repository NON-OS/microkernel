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

//! Reading and writing whole device blocks, each run over one BOT command.

use super::bot::command;
use super::types::{Data, Disk};
use crate::protocol::{E_INVAL, E_IO};
use crate::scsi::{parse_sense, read10, read16, request_sense, write10, write16, Sense};
use crate::span::needs_cdb16;
use crate::state::State;

const SENSE_LEN: u8 = 18;

/// The sense of the last CHECK CONDITION; asking also clears it.
pub(super) fn sense(disk: &Disk, state: &mut State) -> Result<Sense, i32> {
    let mut raw = [0u8; SENSE_LEN as usize];
    match command(disk, state, request_sense(SENSE_LEN), Data::In(&mut raw))? {
        (0, _) => parse_sense(&raw).ok_or(E_IO),
        _ => Err(E_IO),
    }
}

/// The sense key of the last CHECK CONDITION; asking also clears it.
pub(super) fn sense_key(disk: &Disk, state: &mut State) -> Result<u8, i32> {
    sense(disk, state).map(|s| s.sense_key)
}

/// A good status with every byte moved, or `E_IO` with the condition cleared.
pub(super) fn whole(disk: &Disk, state: &mut State, outcome: (u8, u32)) -> Result<(), i32> {
    match outcome {
        (0, 0) => Ok(()),
        (0, _) => Err(E_IO),
        _ => sense_key(disk, state).and(Err(E_IO)),
    }
}

/// Read the device blocks `out` holds from block `lba`.
pub fn read(disk: &Disk, state: &mut State, lba: u64, out: &mut [u8]) -> Result<(), i32> {
    let blocks = whole_blocks(disk, out.len())?;
    let cdb = if needs_cdb16(lba, blocks) {
        read16(lba, blocks)
    } else {
        read10(lba as u32, blocks as u16)
    };
    let outcome = command(disk, state, cdb, Data::In(out))?;
    whole(disk, state, outcome)
}

/// Write `data`, whole device blocks, at block `lba`.
pub fn write(disk: &Disk, state: &mut State, lba: u64, data: &[u8]) -> Result<(), i32> {
    let blocks = whole_blocks(disk, data.len())?;
    let cdb = if needs_cdb16(lba, blocks) {
        write16(lba, blocks)
    } else {
        write10(lba as u32, blocks as u16)
    };
    let outcome = command(disk, state, cdb, Data::Out(data))?;
    whole(disk, state, outcome)
}

fn whole_blocks(disk: &Disk, bytes: usize) -> Result<u32, i32> {
    let len = disk.block_len as usize;
    if len == 0 || bytes == 0 || !bytes.is_multiple_of(len) {
        return Err(E_INVAL);
    }
    u32::try_from(bytes / len).map_err(|_| E_INVAL)
}
