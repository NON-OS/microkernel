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

//! The job queue for a plan, in the order it lands:
//!
//! 1. the old tables and the kernel's markers wiped: a write that stops
//!    before 4 leaves nothing the kernel reads as NONOS, and one that stops
//!    before 6 leaves no partition table, old or new;
//! 2. the ESP's volume, every byte;
//! 3. the data volume's header ring and the key header, zeroed;
//! 4. the store, its header sector last;
//! 5. the disk plan;
//! 6. the partition table, backup first, primary header last.
//!
//! The read-back covers 2 to 6. The wipe is overwritten by what follows it.

use alloc::vec::Vec;

use nonos_disk_map::{PLAN_LBA, STORE_BASE_LBA};

use super::job::Job;
use super::plan::Plan;
use super::queue_esp::esp_jobs;
use super::queue_state::state_jobs;
use crate::gpt::table;

pub struct Queue<'a> {
    pub jobs: Vec<Job<'a>>,
    /// The first job the read-back covers.
    pub kept_from: usize,
    /// The first job of the partition table.
    pub table_from: usize,
}

pub fn queue<'a>(plan: &Plan<'a>) -> Queue<'a> {
    let layout = &plan.layout;
    let mut jobs = Vec::new();
    for (lba, sectors) in
        [(0, 2), (layout.backup_header_lba, 1), (STORE_BASE_LBA, 1), (PLAN_LBA, 1)]
    {
        jobs.push(Job::owned(lba, alloc::vec![0u8; sectors * crate::sink::SECTOR_SIZE]));
    }
    let kept_from = jobs.len();
    esp_jobs(plan, &mut jobs);
    state_jobs(plan, &mut jobs);
    let table_from = jobs.len();
    for (lba, bytes) in table(layout, plan.ids.disk, &plan.ids.partitions) {
        jobs.push(Job::shared(lba, bytes));
    }
    Queue { jobs, kept_from, table_from }
}
