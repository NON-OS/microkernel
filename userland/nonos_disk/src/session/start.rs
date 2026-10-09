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

//! Starting a session, and what a screen asks of one while it runs.

use super::plan::Plan;
use super::queue::queue;
use super::step::Session;

impl<'a> Session<'a> {
    pub fn new(plan: Plan<'a>) -> Session<'a> {
        let q = queue(&plan);
        let total = q.jobs.iter().map(|j| j.len() as u64).sum();
        let (jobs, kept_from, table_from) = (q.jobs, q.kept_from, q.table_from);
        Session {
            plan,
            jobs,
            kept_from,
            table_from,
            next: 0,
            offset: 0,
            done: 0,
            total,
            last: None,
        }
    }

    pub fn total_bytes(&self) -> u64 {
        self.total
    }

    /// Bytes the sink has taken so far.
    pub fn done_bytes(&self) -> u64 {
        self.done
    }

    /// The request the session last handed the sink: after a failure, the
    /// one that failed.
    pub fn last_attempt(&self) -> Option<super::step::Attempt> {
        self.last
    }

    /// True until the first byte of the partition table is sent: stopping
    /// then leaves a disk with no table, old or new.
    pub fn stoppable(&self) -> bool {
        self.next < self.table_from || (self.next == self.table_from && self.offset == 0)
    }

    /// The plan this session writes, for a screen that describes it.
    pub fn plan(&self) -> &Plan<'a> {
        &self.plan
    }
}
