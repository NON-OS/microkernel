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

//! The size of a device and keeping its writes: READ CAPACITY(10) or (16)
//! and SYNCHRONIZE CACHE(10).

use super::bot::command;
use super::sense::{sense, whole};
use super::types::{Data, Disk};
use crate::protocol::E_IO;
use crate::scsi::{
    parse_capacity, parse_capacity16, read_capacity10, read_capacity16, CAPACITY16_DATA_LEN,
    CAPACITY_DATA_LEN,
};
use crate::state::State;

const ILLEGAL_REQUEST: u8 = 0x05;
/// READ CAPACITY(10)'s last LBA when the device has more blocks than it
/// can say: READ CAPACITY(16) has the count.
const LAST_LBA_TOO_BIG: u32 = u32::MAX;

/// The block count and block length. A device past 2 TiB of 512-byte
/// blocks reports a last LBA of 0xFFFFFFFF to READ CAPACITY(10) and is asked
/// READ CAPACITY(16); it used to be served for its first 2^32 blocks only.
pub fn capacity(disk: &Disk, state: &mut State) -> Result<(u64, u32), i32> {
    let mut raw = [0u8; CAPACITY_DATA_LEN];
    let outcome = command(disk, state, read_capacity10(), Data::In(&mut raw))?;
    whole(disk, state, outcome)?;
    let c = parse_capacity(&raw).ok_or(E_IO)?;
    if c.last_lba != LAST_LBA_TOO_BIG {
        return Ok((c.block_count(), c.block_len));
    }
    let mut raw = [0u8; CAPACITY16_DATA_LEN];
    let outcome = command(disk, state, read_capacity16(), Data::In(&mut raw))?;
    // A device may return fewer than the 32 bytes asked; the 12 that carry
    // the count and the length are all that is needed.
    match outcome {
        (0, residue) if residue as usize <= CAPACITY16_DATA_LEN - 12 => {}
        other => whole(disk, state, other)?,
    }
    parse_capacity16(&raw).ok_or(E_IO)
}

/// SYNCHRONIZE CACHE(10). A device that answers ILLEGAL REQUEST has no
/// cache it lets the host flush, and its writes are already on the medium
/// as far as it will say; that is taken as done.
pub fn sync_cache(disk: &Disk, state: &mut State) -> Result<(), i32> {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x35;
    match command(disk, state, (cdb, 10), Data::None)? {
        (0, _) => Ok(()),
        _ if sense(disk, state)?.is_some_and(|s| s.sense_key == ILLEGAL_REQUEST) => Ok(()),
        _ => Err(E_IO),
    }
}
