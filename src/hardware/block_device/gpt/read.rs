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

//! The table off a disk: the protective MBR at LBA 0, the primary header
//! at LBA 1, the backup at the last LBA, then the array of the header
//! believed, read in requests no larger than every driver takes. Nothing
//! here writes the disk.

use alloc::vec::Vec;

use super::choose::{choose, Table};
use super::header::Header;
use crate::hardware::block_device::backend::Backend;
use crate::hardware::block_device::found::Found;
use crate::hardware::block_device::read::read_on;
use crate::hardware::block_device::BlockDeviceError;

/// Sixty-four sectors, what every block driver takes in one request.
const CHUNK: usize = 32 * 1024;

/// What `b`'s disk of `sectors` names. A read the disk refused is the
/// caller's to judge, as for any other sector; a last LBA the disk cannot
/// serve is a disk without a backup, not a disk without a table.
pub(crate) fn table(b: Backend, sectors: u64) -> Result<Table, BlockDeviceError> {
    let (mut lba0, mut lba1, mut last) = ([0u8; 512], [0u8; 512], [0u8; 512]);
    read_on(b, 0, &mut lba0)?;
    read_on(b, 1, &mut lba1)?;
    if let Err(e) = read_on(b, sectors - 1, &mut last) {
        if matches!(Found::of(e), Found::Refused(_)) {
            return Err(e);
        }
        last = [0u8; 512];
    }
    choose((&lba0, &lba1, &last), sectors, |h| array(b, h))
}

fn array(b: Backend, h: &Header) -> Result<Vec<u8>, BlockDeviceError> {
    let len = h.array_sectors() as usize * 512;
    let mut array = Vec::new();
    // Memory short now is not a disk without a table: ask again later.
    array.try_reserve_exact(len).map_err(|_| BlockDeviceError::NotReady)?;
    array.resize(len, 0u8);
    for (i, chunk) in array.chunks_mut(CHUNK).enumerate() {
        read_on(b, h.entries_lba + (i * CHUNK / 512) as u64, chunk)?;
    }
    Ok(array)
}
