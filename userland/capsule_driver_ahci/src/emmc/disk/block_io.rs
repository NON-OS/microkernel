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

//! Capacity, sector read and write through the data buffer, and flush.

use super::super::env::{Clock, Log, Mmio};
use super::super::error::{EmmcError, EmmcResult};
use super::super::mmc::io;
use super::{EmmcDisk, MAX_SECTORS, SECTOR_SIZE};

impl<M: Mmio, C: Clock, L: Log> EmmcDisk<M, C, L> {
    pub fn capacity_sectors(&self) -> u64 {
        self.card.sectors
    }

    /// The most sectors one request moves.
    pub fn max_sectors(&self) -> u32 {
        core::cmp::min(MAX_SECTORS as usize, self.data.len / SECTOR_SIZE) as u32
    }

    /// Move `n` sectors at `lba` between the card and the data buffer. A
    /// server that copies straight from or into `data` uses this.
    pub fn transfer(&mut self, lba: u64, n: u32, write: bool) -> EmmcResult<()> {
        if n > self.max_sectors() {
            return Err(EmmcError::OutOfRange);
        }
        io::transfer(&mut self.host, &self.card, self.data, lba, n, write)
    }

    /// Read `n` sectors at `lba` into the start of `out`.
    pub fn read(&mut self, lba: u64, n: u32, out: &mut [u8]) -> EmmcResult<()> {
        let bytes = n as usize * SECTOR_SIZE;
        if out.len() < bytes {
            return Err(EmmcError::OutOfRange);
        }
        self.transfer(lba, n, false)?;
        if !self.data.get(0, &mut out[..bytes]) {
            return Err(EmmcError::OutOfRange);
        }
        Ok(())
    }

    /// Write `src`, exactly `n` sectors, at `lba`.
    pub fn write(&mut self, lba: u64, n: u32, src: &[u8]) -> EmmcResult<()> {
        if src.len() != n as usize * SECTOR_SIZE || n > self.max_sectors() {
            return Err(EmmcError::OutOfRange);
        }
        if !self.data.put(0, src) {
            return Err(EmmcError::OutOfRange);
        }
        self.transfer(lba, n, true)
    }

    pub fn flush(&mut self) -> EmmcResult<()> {
        io::flush(&mut self.host, &self.card)
    }
}
