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

//! Reading and writing whole blocks, each over one BOT command.

use super::bot::command;
use super::types::{Data, Disk};
use crate::protocol::{BLOCK_BYTES, E_IO};
use crate::scsi::{parse_sense, read10, request_sense, write10};
use crate::state::State;

const SENSE_LEN: u8 = 18;

/// The sense key of the last CHECK CONDITION; asking also clears it.
pub(super) fn sense_key(disk: &Disk, state: &mut State) -> Result<u8, i32> {
    let mut raw = [0u8; SENSE_LEN as usize];
    match command(disk, state, request_sense(SENSE_LEN), Data::In(&mut raw))? {
        (0, _) => parse_sense(&raw).map(|s| s.sense_key).ok_or(E_IO),
        _ => Err(E_IO),
    }
}

/// A good status with every byte moved, or `E_IO` with the condition cleared.
pub(super) fn whole(disk: &Disk, state: &mut State, outcome: (u8, u32)) -> Result<(), i32> {
    match outcome {
        (0, 0) => Ok(()),
        (0, _) => Err(E_IO),
        _ => sense_key(disk, state).and(Err(E_IO)),
    }
}

pub fn read(disk: &Disk, state: &mut State, lba: u32, out: &mut [u8]) -> Result<(), i32> {
    let blocks = (out.len() / BLOCK_BYTES as usize) as u16;
    let outcome = command(disk, state, read10(lba, blocks), Data::In(out))?;
    whole(disk, state, outcome)
}

pub fn write(disk: &Disk, state: &mut State, lba: u32, data: &[u8]) -> Result<(), i32> {
    let blocks = (data.len() / BLOCK_BYTES as usize) as u16;
    let outcome = command(disk, state, write10(lba, blocks), Data::Out(data))?;
    whole(disk, state, outcome)
}
