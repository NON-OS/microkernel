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

//! READ, WRITE and SYNCHRONIZE CACHE on a unit, through the data region.
//! The caller holds a request to `max_blocks` and to the unit's size.

use super::ufs::Ufs;
use super::unit::Unit;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::scsi::cdb::{rw, sync_cache};
use crate::ufs::scsi::exec::{exec, Data};
use crate::ufs::time::SCSI_MS;

pub fn read<H: Hw>(
    hw: &H,
    d: &Ufs,
    u: &Unit,
    lba: u64,
    n: u32,
    out: &mut [u8],
) -> Result<(), UfsError> {
    let bytes = n as usize * u.block_len as usize;
    exec(hw, &d.ctx, u.lun, rw(false, lba, n), Data::In(bytes), SCSI_MS)?;
    d.ctx.lists.data.get(0, &mut out[..bytes]);
    Ok(())
}

pub fn write<H: Hw>(
    hw: &H,
    d: &Ufs,
    u: &Unit,
    lba: u64,
    n: u32,
    src: &[u8],
) -> Result<(), UfsError> {
    let bytes = n as usize * u.block_len as usize;
    // Before the bounce buffer is touched: an old READ may still land in it.
    if d.ctx.slot_held(hw) {
        return Err(UfsError::Host);
    }
    d.ctx.lists.data.put(0, &src[..bytes]);
    exec(hw, &d.ctx, u.lun, rw(true, lba, n), Data::Out(bytes), SCSI_MS)
}

pub fn flush<H: Hw>(hw: &H, d: &Ufs, u: &Unit) -> Result<(), UfsError> {
    exec(hw, &d.ctx, u.lun, sync_cache(), Data::None, SCSI_MS)
}
