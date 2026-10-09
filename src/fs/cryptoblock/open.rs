// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

//! Opening a sealed sector: into a block returned, or into one the caller
//! holds, such as its place in a larger read.

use super::constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use super::sector_open::open_sealed;
use super::CryptoBlockError;

pub fn open(
    key: &[u8; 32],
    lba: u64,
    sector: &[u8; SECTOR_BYTES],
) -> Result<[u8; PLAIN_BLOCK_BYTES], CryptoBlockError> {
    let mut out = [0u8; PLAIN_BLOCK_BYTES];
    open_into(key, lba, sector, &mut out)?;
    Ok(out)
}

/// Open the sector sealed at `lba` into `out`. Nothing is written to `out`
/// unless the sector's tag matches.
pub fn open_into(
    key: &[u8; 32],
    lba: u64,
    sector: &[u8; SECTOR_BYTES],
    out: &mut [u8; PLAIN_BLOCK_BYTES],
) -> Result<(), CryptoBlockError> {
    /*
     * One sector is the unit of work of every volume read and write, which
     * run inside system calls with interrupts masked and can span thousands
     * of sectors. Answer any TLB shootdown here; only kernel buffers are in
     * hand, so nothing translated from user memory is held across it.
     */
    crate::smp::serve_shootdowns();
    if open_sealed(key, lba, sector, out) {
        Ok(())
    } else {
        Err(CryptoBlockError::AuthenticationFailed)
    }
}
