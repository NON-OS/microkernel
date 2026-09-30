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
//! order the queue holds them, which puts every byte of every partition on
//! the disk before the table that names them, and the flush after both.

use alloc::vec::Vec;

use super::job::Job;
use super::plan::Plan;
use super::progress::Progress;
use crate::sink::{BlockSink, SECTOR_SIZE};
use crate::writer::WriteError;

pub struct Session<'a> {
    pub(super) plan: Plan<'a>,
    pub(super) jobs: Vec<Job<'a>>,
    pub(super) kept_from: usize,
    pub(super) table_from: usize,
    pub(super) next: usize,
    pub(super) offset: usize,
    pub(super) done: u64,
    pub(super) total: u64,
}

impl<'a> Session<'a> {
    /// Write up to `budget` bytes of the current job, rounded down to whole
    /// sectors and never less than one, then report. Call until `Done`.
    pub fn step(
        &mut self,
        sink: &mut dyn BlockSink,
        budget: usize,
    ) -> Result<Progress<'a>, WriteError> {
        let Some(job) = self.jobs.get(self.next) else { return self.finish(sink) };
        let remaining = job.len() - self.offset;
        let n = remaining.min((budget / SECTOR_SIZE).max(1) * SECTOR_SIZE);
        let lba = job.lba + (self.offset / SECTOR_SIZE) as u64;
        sink.write_at(lba, &job.bytes()[self.offset..self.offset + n])?;
        self.offset += n;
        self.done += n as u64;
        if self.offset == job.len() {
            self.next += 1;
            self.offset = 0;
            if self.next == self.jobs.len() {
                return Ok(Progress::TableWritten);
            }
        }
        Ok(Progress::Writing { done: self.done, total: self.total })
    }
}
