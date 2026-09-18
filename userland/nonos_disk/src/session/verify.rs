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

//! The read-back, in steps: every file compared against its source, one
//! budget of sectors at a time. Owns its list of runs, so a caller can hold
//! it across frames without holding the receipt it came from.

use alloc::vec::Vec;

use crate::sink::{BlockSink, SECTOR_SIZE};
use crate::writer::{FileRun, Receipt, WriteError};

/// Reads every file back in steps of `budget` bytes and compares.
pub struct Verifier<'a> {
    files: Vec<FileRun<'a>>,
    file: usize,
    offset: usize,
    pub checked: u64,
}

impl<'a> Verifier<'a> {
    pub fn new(receipt: &Receipt<'a>) -> Self {
        Verifier { files: receipt.files.clone(), file: 0, offset: 0, checked: 0 }
    }

    pub fn total_bytes(&self) -> u64 {
        self.files.iter().map(|f| f.data.len() as u64).sum()
    }

    /// `Ok(true)` while there is more to read, `Ok(false)` when every file
    /// matched, `Err` at the first sector that did not.
    pub fn step(&mut self, sink: &mut dyn BlockSink, budget: usize) -> Result<bool, WriteError> {
        let Some(f) = self.files.get(self.file).copied() else { return Ok(false) };
        let want = (f.data.len() - self.offset).min((budget / SECTOR_SIZE).max(1) * SECTOR_SIZE);
        let sectors = want.div_ceil(SECTOR_SIZE);
        let lba = f.lba + (self.offset / SECTOR_SIZE) as u64;
        let mut buf = alloc::vec![0u8; sectors * SECTOR_SIZE];
        sink.read_at(lba, &mut buf)?;
        let src = &f.data[self.offset..self.offset + want];
        if let Some(bad) = buf[..want].iter().zip(src).position(|(a, b)| a != b) {
            return Err(WriteError::Mismatch { lba: lba + (bad / SECTOR_SIZE) as u64 });
        }
        self.offset += want;
        self.checked += want as u64;
        if self.offset >= f.data.len() {
            self.file += 1;
            self.offset = 0;
        }
        Ok(true)
    }
}
