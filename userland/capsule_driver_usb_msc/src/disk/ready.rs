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

//! Bringing a device up and keeping its writes: TEST UNIT READY, READ
//! CAPACITY(10) and SYNCHRONIZE CACHE(10).

use nonos_libc::mk_yield;

use super::bot::command;
use super::scsi_io::{sense_key, whole};
use super::types::{Data, Disk};
use crate::protocol::E_IO;
use crate::scsi::{parse_capacity, read_capacity10, test_unit_ready, CAPACITY_DATA_LEN};
use crate::state::State;

const ILLEGAL_REQUEST: u8 = 0x05;
/// TEST UNIT READY tries: a stick reports UNIT ATTENTION once after reset.
const READY_TRIES: u32 = 10;

pub fn unit_ready(disk: &Disk, state: &mut State) -> Result<(), i32> {
    for _ in 0..READY_TRIES {
        if command(disk, state, test_unit_ready(), Data::None)?.0 == 0 {
            return Ok(());
        }
        sense_key(disk, state)?;
        mk_yield();
    }
    Err(E_IO)
}

/// The block count and block length. A device whose last LBA does not fit
/// READ CAPACITY(10) reports 0xFFFFFFFF, and is served for its first 2^32
/// blocks, all READ(10) can address.
pub fn capacity(disk: &Disk, state: &mut State) -> Result<(u64, u32), i32> {
    let mut raw = [0u8; CAPACITY_DATA_LEN];
    let outcome = command(disk, state, read_capacity10(), Data::In(&mut raw))?;
    whole(disk, state, outcome)?;
    let c = parse_capacity(&raw).ok_or(E_IO)?;
    Ok((c.block_count(), c.block_len))
}

/// SYNCHRONIZE CACHE(10). A device that answers ILLEGAL REQUEST has no
/// cache it lets the host flush, and its writes are already on the medium
/// as far as it will say; that is taken as done.
pub fn sync_cache(disk: &Disk, state: &mut State) -> Result<(), i32> {
    let mut cdb = [0u8; 16];
    cdb[0] = 0x35;
    match command(disk, state, (cdb, 10), Data::None)? {
        (0, _) => Ok(()),
        _ if sense_key(disk, state)? == ILLEGAL_REQUEST => Ok(()),
        _ => Err(E_IO),
    }
}
