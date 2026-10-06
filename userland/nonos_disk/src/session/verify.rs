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

//! The read-back, in steps: every sector that stays written compared with
//! what was sent, the tables, the store, the plan, the key header, the ring
//! and every byte of the ESP, one budget at a time. Owns its jobs, so a
//! caller can hold it across frames without holding the receipt.

use alloc::vec::Vec;

use super::job::Job;
use crate::sink::{BlockSink, SECTOR_SIZE};
use crate::writer::{Receipt, WriteError};

pub struct Verifier<'a> {
    jobs: Vec<Job<'a>>,
    job: usize,
    offset: usize,
    pub checked: u64,
}

impl<'a> Verifier<'a> {
    pub fn new(receipt: &Receipt<'a>) -> Self {
        Verifier { jobs: receipt.written.clone(), job: 0, offset: 0, checked: 0 }
    }

    pub fn total_bytes(&self) -> u64 {
        self.jobs.iter().map(|j| j.len() as u64).sum()
    }

    /// `Ok(true)` while there is more to read, `Ok(false)` when every sector
    /// matched, `Err` at the first sector that did not.
    pub fn step(&mut self, sink: &mut dyn BlockSink, budget: usize) -> Result<bool, WriteError> {
        let Some(j) = self.jobs.get(self.job) else { return Ok(false) };
        let want = (j.len() - self.offset).min((budget / SECTOR_SIZE).max(1) * SECTOR_SIZE);
        let lba = j.lba + (self.offset / SECTOR_SIZE) as u64;
        let mut buf = alloc::vec![0u8; want.div_ceil(SECTOR_SIZE) * SECTOR_SIZE];
        sink.read_at(lba, &mut buf)?;
        let sent = &j.bytes()[self.offset..self.offset + want];
        if let Some(bad) = buf[..want].iter().zip(sent).position(|(a, b)| a != b) {
            return Err(WriteError::Mismatch { lba: lba + (bad / SECTOR_SIZE) as u64 });
        }
        self.offset += want;
        self.checked += want as u64;
        if self.offset >= j.len() {
            self.job += 1;
            self.offset = 0;
        }
        Ok(true)
    }
}
