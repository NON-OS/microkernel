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

//! Reading and writing whole sectors: `lba_le64, sectors_le32`, and for a
//! write the data. Sectors are 512 bytes whatever the device's logical block
//! (`crate::span`): a request is moved as the device blocks that cover it,
//! and a write that covers part of a block reads the block first.

use super::sectors::{read_sectors, write_sectors};
use crate::disk::Disk;
use crate::protocol::{
    BLK_HEADER_LEN, BLK_MAX_SECTORS, E_INVAL, E_IO, E_MSGSIZE, E_NOTSUP, E_NXIO, E_TIMEDOUT,
};
use crate::span::{sectors, sectors_per_block, span, SECTOR_BYTES};
use crate::state::State;

pub enum Rw<'a> {
    /// Read into the reply's data area.
    Read(&'a mut [u8]),
    Write,
}

/// Returns the bytes moved.
pub fn read_write(state: &mut State, disk: &Disk, rw: Rw, body: &[u8]) -> Result<usize, i32> {
    if body.len() < BLK_HEADER_LEN {
        return Err(E_INVAL);
    }
    let lba = u64::from_le_bytes(body[0..8].try_into().map_err(|_| E_INVAL)?);
    let count = u32::from_le_bytes(body[8..12].try_into().map_err(|_| E_INVAL)?);
    let per_block = sectors_per_block(disk.block_len).ok_or(E_NOTSUP)?;
    if count == 0 {
        return Err(E_INVAL);
    }
    if count > BLK_MAX_SECTORS {
        return Err(E_MSGSIZE);
    }
    let end = lba.checked_add(count as u64).ok_or(E_NXIO)?;
    if end > sectors(disk.blocks, per_block) {
        return Err(E_NXIO);
    }
    let sp = span(lba, count, per_block).ok_or(E_INVAL)?;
    let bytes = count as usize * SECTOR_BYTES as usize;
    let done = match rw {
        Rw::Read(out) => read_sectors(state, disk, sp, &mut out[..bytes]),
        Rw::Write if body.len() == BLK_HEADER_LEN + bytes => {
            write_sectors(state, disk, sp, &body[BLK_HEADER_LEN..])
        }
        Rw::Write => return Err(E_INVAL),
    };
    // A request that ran out of its time says so; any other failure is the
    // device's.
    done.map(|_| bytes).map_err(|e| if e == E_TIMEDOUT { e } else { E_IO })
}
