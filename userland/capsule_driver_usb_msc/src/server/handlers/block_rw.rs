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
//! write the data. Only a device with 512-byte blocks is served; any other
//! is refused by name rather than read at the wrong offsets.

use crate::disk::{read, write, Disk};
use crate::protocol::{
    BLK_HEADER_LEN, BLK_MAX_SECTORS, BLOCK_BYTES, E_INVAL, E_IO, E_MSGSIZE, E_NOTSUP, E_NXIO,
};
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
    if disk.block_len != BLOCK_BYTES {
        return Err(E_NOTSUP);
    }
    if count == 0 {
        return Err(E_INVAL);
    }
    if count > BLK_MAX_SECTORS {
        return Err(E_MSGSIZE);
    }
    /*
     * READ(10) and WRITE(10) carry a 32-bit LBA; nothing past what they
     * address, or past the device's end, is asked for.
     */
    let end = lba.checked_add(count as u64).ok_or(E_NXIO)?;
    if end > disk.blocks || end > 1u64 << 32 {
        return Err(E_NXIO);
    }
    let bytes = count as usize * BLOCK_BYTES as usize;
    let done = match rw {
        Rw::Read(out) => read(disk, state, lba as u32, &mut out[..bytes]),
        Rw::Write if body.len() == BLK_HEADER_LEN + bytes => {
            write(disk, state, lba as u32, &body[BLK_HEADER_LEN..])
        }
        Rw::Write => return Err(E_INVAL),
    };
    done.map(|_| bytes).map_err(|_| E_IO)
}
