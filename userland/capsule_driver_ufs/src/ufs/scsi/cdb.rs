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

//! Command descriptor blocks, big-endian as SCSI is, in a 16-byte buffer
//! with their length.

use super::codes::*;

pub type Cdb = ([u8; 16], usize);

pub(super) fn cdb(len: usize, f: impl FnOnce(&mut [u8; 16])) -> Cdb {
    let mut b = [0u8; 16];
    f(&mut b);
    (b, len)
}

pub fn test_unit_ready() -> Cdb {
    cdb(6, |b| b[0] = TEST_UNIT_READY)
}

/// READ or WRITE: the 10-byte form while the LBA and count fit it, which
/// every LU takes, the 16-byte form past that.
pub fn rw(write: bool, lba: u64, blocks: u32) -> Cdb {
    if lba <= u32::MAX as u64 && blocks <= u16::MAX as u32 {
        return cdb(10, |b| {
            b[0] = if write { WRITE_10 } else { READ_10 };
            b[2..6].copy_from_slice(&(lba as u32).to_be_bytes());
            b[7..9].copy_from_slice(&(blocks as u16).to_be_bytes());
        });
    }
    cdb(16, |b| {
        b[0] = if write { WRITE_16 } else { READ_16 };
        b[2..10].copy_from_slice(&lba.to_be_bytes());
        b[10..14].copy_from_slice(&blocks.to_be_bytes());
    })
}

/// SYNCHRONIZE CACHE(10) over the whole unit: LBA 0, count 0.
pub fn sync_cache() -> Cdb {
    cdb(10, |b| b[0] = SYNCHRONIZE_CACHE_10)
}

/// REPORT LUNS, select report 0: the logical units, no well-known ones.
pub fn report_luns(alloc: u32) -> Cdb {
    cdb(12, |b| {
        b[0] = REPORT_LUNS;
        b[6..10].copy_from_slice(&alloc.to_be_bytes());
    })
}
