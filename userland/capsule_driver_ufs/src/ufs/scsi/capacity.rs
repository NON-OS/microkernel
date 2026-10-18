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

//! How big a unit is: READ CAPACITY(10), and (16) when (10) says the last
//! LBA does not fit 32 bits. Blocks must be 512 or 4096 bytes, the sizes UFS
//! logical units are made with (bLogicalBlockSize 9 or 12).

use super::cdb::{cdb, Cdb};
use super::codes::*;
use super::exec::{exec, Data};
use crate::ufs::ctx::Ctx;
use crate::ufs::env::Hw;
use crate::ufs::error::UfsError;
use crate::ufs::facts::bounds::SCSI_MS;

const RC16_ALLOC: usize = 32;

/// The block count and the block size.
pub fn read<H: Hw>(hw: &H, c: &Ctx, lun: u8) -> Result<(u64, u32), UfsError> {
    let mut b = [0u8; RC16_ALLOC];
    exec(hw, c, lun, cdb(10, |b| b[0] = READ_CAPACITY_10), Data::In(8), SCSI_MS)?;
    c.lists.data.get(0, &mut b[..8]);
    let (blocks, len) = match parse_10(&b) {
        Some(v) => v,
        None => {
            exec(hw, c, lun, rc16(), Data::In(RC16_ALLOC), SCSI_MS)?;
            c.lists.data.get(0, &mut b);
            parse_16(&b)
        }
    };
    match len {
        // A unit whose bytes overflow is no disk: its last LBA was a lie.
        512 | 4096 if blocks.checked_mul(len as u64).is_none() => Err(UfsError::Capacity(blocks)),
        512 | 4096 => Ok((blocks, len)),
        _ => Err(UfsError::BlockSize(len)),
    }
}

fn rc16() -> Cdb {
    cdb(16, |b| {
        b[0] = SERVICE_ACTION_IN_16;
        b[1] = SA_READ_CAPACITY_16;
        b[10..14].copy_from_slice(&(RC16_ALLOC as u32).to_be_bytes());
    })
}

/// None when the last LBA reads 0xFFFFFFFF, the sign to ask (16).
pub fn parse_10(b: &[u8]) -> Option<(u64, u32)> {
    let last = u32::from_be_bytes([b[0], b[1], b[2], b[3]]);
    let len = u32::from_be_bytes([b[4], b[5], b[6], b[7]]);
    (last != u32::MAX).then_some((last as u64 + 1, len))
}

pub fn parse_16(b: &[u8]) -> (u64, u32) {
    let last = u64::from_be_bytes([b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7]]);
    (last.saturating_add(1), u32::from_be_bytes([b[8], b[9], b[10], b[11]]))
}
