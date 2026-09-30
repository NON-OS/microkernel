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
//! Reading ahead: a run of sealed sectors fetched in one device request,
//! each opened only when asked for, against its own LBA.

use alloc::vec::Vec;

use super::constants::{PLAIN_BLOCK_BYTES, SECTOR_BYTES};
use super::map_block::map_block_error;
use super::open::open;
use super::pending::{drain, RUN_SECTORS};
use super::window::{device_lba, window_sectors};
use super::CryptoBlockError;

/// Sealed sectors from volume LBA `start`, as fetched. Nothing in it is
/// trusted until `open` has checked it.
pub struct ReadAhead {
    start: u64,
    sealed: Vec<u8>,
}

impl ReadAhead {
    pub const fn new() -> Self {
        ReadAhead { start: 0, sealed: Vec::new() }
    }

    /// Whether the run fetched last holds the sector at `lba`.
    pub fn holds(&self, lba: u64) -> bool {
        lba >= self.start && lba - self.start < (self.sealed.len() / SECTOR_BYTES) as u64
    }

    /// The block at `lba`, fetching it and the run after it on a miss.
    pub fn read(
        &mut self,
        key: &[u8; 32],
        lba: u64,
    ) -> Result<[u8; PLAIN_BLOCK_BYTES], CryptoBlockError> {
        if !self.holds(lba) {
            self.fetch(lba)?;
        }
        let at = (lba - self.start) as usize * SECTOR_BYTES;
        let mut sector = [0u8; SECTOR_BYTES];
        sector.copy_from_slice(&self.sealed[at..at + SECTOR_BYTES]);
        open(key, lba, &sector)
    }

    fn fetch(&mut self, lba: u64) -> Result<(), CryptoBlockError> {
        drain()?;
        let at = device_lba(lba)?;
        let left = window_sectors().ok_or(CryptoBlockError::NoWindow)? - lba;
        let count = left.min(RUN_SECTORS as u64) as usize;
        self.sealed.clear();
        self.sealed.resize(count * SECTOR_BYTES, 0);
        let got = crate::hardware::block_device::read(at, &mut self.sealed);
        if let Err(e) = got {
            self.sealed.clear();
            return Err(map_block_error(e));
        }
        self.start = lba;
        Ok(())
    }
}
