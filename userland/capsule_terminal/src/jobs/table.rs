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

use alloc::vec::Vec;

use super::env::JobEnv;
use super::work::JobWork;

#[path = "table_reap.rs"]
mod table_reap;
#[path = "table_record.rs"]
mod table_record;
pub use table_record::{JobProgress, JobRecord, JobState};

// Every job lives on the single event-loop thread; the table is stepped
// from `on_tick`, never shared across threads or interrupt context.
pub struct JobTable {
    jobs: Vec<JobRecord>,
    next_id: u32,
    /// The plain `linux` run that may ask to start again with the network.
    pub net_ask: Option<super::net_ask::NetAsk>,
}

impl JobTable {
    pub fn new() -> Self {
        Self { jobs: Vec::new(), next_id: 1, net_ask: None }
    }

    pub fn add(&mut self, cmdline: &[u8], background: bool, work: JobWork, env: JobEnv) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.jobs.push(JobRecord {
            id,
            cmdline: cmdline.to_vec(),
            background,
            cancel: false,
            status: 0,
            state: JobState::Running,
            env,
            work,
        });
        id
    }

    pub fn foreground(&self) -> Option<u32> {
        self.jobs.iter().rev().find(|j| !j.background && j.state == JobState::Running).map(|j| j.id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut JobRecord> {
        self.jobs.iter_mut().find(|j| j.id == id)
    }

    pub fn get(&self, id: u32) -> Option<&JobRecord> {
        self.jobs.iter().find(|j| j.id == id)
    }

    pub fn iter(&self) -> impl Iterator<Item = &JobRecord> {
        self.jobs.iter()
    }
}
