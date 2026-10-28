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

//! The job queue for an update, in the order it lands:
//!
//! 1. the boot partition's reserved area and the store's header sector
//!    wiped: until 4, no firmware reads the boot partition as a FAT volume
//!    and loads a loader half written, and no kernel or vfs reads the old
//!    store's table over payloads the new store is overwriting;
//! 2. the boot partition's FATs, directories and files;
//! 3. the store, its header sector last;
//! 4. the boot partition's reserved area, which makes it a volume again.
//!
//! Nothing goes to the protective MBR, either table, NONOS-PLAN (the disk
//! plan, the key header) or any sector of NONOS-DATA, the header ring
//! included. So a write that stops anywhere leaves the data volume and
//! everything an update checks as they were: the disk takes the update
//! again, or a full install, as before. The read-back covers 2 to 4, which
//! write over every sector the wipe did.

use alloc::vec::Vec;

use nonos_disk_map::STORE_BASE_LBA;

use super::job::Job;
use super::plan::Plan;
use super::queue::Queue;
use super::queue_esp::esp_jobs;
use super::queue_state::store_jobs;
use crate::fat32::RESERVED_SECTORS;
use crate::sink::SECTOR_SIZE;

pub fn keep_queue<'a>(plan: &Plan<'a>) -> Queue<'a> {
    let reserved = RESERVED_SECTORS as usize * SECTOR_SIZE;
    let mut jobs: Vec<Job<'a>> = alloc::vec![
        /*
         * Both wipes are in the system part the update writes in full, and
         * each is written again below: the reserved area as the last job,
         * the store's header as the store's last. Neither is the plan
         * partition or the data partition. The kernel still finds the disk
         * by the plan's magic, which stays.
         */
        Job::owned(plan.layout.esp.first, alloc::vec![0u8; reserved]),
        Job::owned(STORE_BASE_LBA, alloc::vec![0u8; SECTOR_SIZE]),
    ];
    let kept_from = jobs.len();
    esp_jobs(plan, &mut jobs);
    /* `esp_jobs` queues the reserved area first; it goes last here. */
    let boot = jobs.remove(kept_from);
    store_jobs(plan, &mut jobs);
    /*
     * Stopping is offered until this last write starts (`stoppable`), as
     * an install offers it until the table: what it leaves is a disk the
     * update takes again. Past that point the update is one write from done.
     */
    let table_from = jobs.len();
    jobs.push(boot);
    Queue { jobs, kept_from, table_from }
}
