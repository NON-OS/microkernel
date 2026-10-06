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

//! The sectors the kernel reads at boot: the data volume's header ring and
//! the key header zeroed, the store, and the disk plan.

use alloc::rc::Rc;
use alloc::vec::Vec;

use nonos_disk_map::{HEADER_RING_SECTORS, KEY_LBA, PLAN_LBA, STORE_BASE_LBA};

use super::job::Job;
use super::plan::Plan;
use crate::plan_sector::plan_sector;
use crate::sink::SECTOR_SIZE;

pub fn state_jobs<'a>(plan: &Plan<'a>, jobs: &mut Vec<Job<'a>>) {
    let zeros = Rc::new(alloc::vec![0u8; HEADER_RING_SECTORS as usize * SECTOR_SIZE]);
    /*
     * The kernel formats a volume only over a ring of zeros. Any other ring,
     * a volume this disk held before included, it neither formats over nor
     * opens under a new key.
     */
    jobs.push(Job::part(plan.layout.data.first, zeros.clone(), 0..zeros.len()));
    /*
     * A key header without its magic says the TPM keys the volume. The
     * first boot that opens the volume writes the real header, then formats.
     */
    jobs.push(Job::part(KEY_LBA, zeros, 0..SECTOR_SIZE));
    /*
     * The header sector last, as vfs commits a table: it is what makes the
     * sectors after it a store.
     */
    let store = plan.store.shared();
    jobs.push(Job::part(STORE_BASE_LBA + 1, store.clone(), SECTOR_SIZE..store.len()));
    jobs.push(Job::part(STORE_BASE_LBA, store, 0..SECTOR_SIZE));
    jobs.push(Job::owned(PLAN_LBA, plan_sector(&plan.layout.data)));
}
