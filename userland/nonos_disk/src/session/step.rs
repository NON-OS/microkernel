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

//! The session: a plan, its job queue, and a cursor. Jobs go down in the
//! order the queue holds them, which puts every byte of the volume on the
//! disk before the table that names it, and the flush after both.

use alloc::vec::Vec;

use super::job::Job;
use super::plan::Plan;
use super::progress::Progress;
use crate::sink::{BlockSink, SECTOR_SIZE};
use crate::writer::WriteError;

pub struct Session<'a> {
    pub(super) plan: Plan<'a>,
    jobs: Vec<Job<'a>>,
    next: usize,
    offset: usize,
    pub(super) done: u64,
    total: u64,
    pub(super) table_written: bool,
}

impl<'a> Session<'a> {
    pub fn new(plan: Plan<'a>) -> Session<'a> {
        let jobs = super::queue::queue(&plan);
        let total = jobs.iter().map(|j| j.len() as u64).sum();
        Session { plan, jobs, next: 0, offset: 0, done: 0, total, table_written: false }
    }

    pub fn total_bytes(&self) -> u64 {
        self.total
    }

    /// Write up to `budget` bytes, rounded down to whole sectors and never
    /// less than one, then report. Call until `Done`.
    pub fn step(
        &mut self,
        sink: &mut dyn BlockSink,
        budget: usize,
    ) -> Result<Progress<'a>, WriteError> {
        if let Some(job) = self.jobs.get(self.next) {
            let remaining = job.len() - self.offset;
            let n = remaining.min((budget / SECTOR_SIZE).max(1) * SECTOR_SIZE);
            let lba = job.lba + (self.offset / SECTOR_SIZE) as u64;
            sink.write_at(lba, &job.bytes()[self.offset..self.offset + n])?;
            self.offset += n;
            self.done += n as u64;
            if self.offset == job.len() {
                self.next += 1;
                self.offset = 0;
            }
            return Ok(Progress::Writing { done: self.done, total: self.total });
        }
        self.finish(sink)
    }
}
