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

//! The job queue for a plan: the volume in the order it lands.

use alloc::vec::Vec;

use super::job::Job;
use super::plan::Plan;
use crate::fat32::{build_fat, build_reserved, encode_dir, Content, FAT_COUNT, RESERVED_SECTORS};
use crate::sink::SECTOR_SIZE;

/// Reserved area, both FATs, every directory, every file: the volume in
/// the order it lands. Directory names that cannot be spelled are caught
/// here, before any write.
pub fn queue<'a>(plan: &Plan<'a>) -> Vec<Job<'a>> {
    let first = plan.layout.esp_first_lba;
    let geo = &plan.geometry;
    let mut jobs = Vec::new();
    let used = plan.placed.data_clusters_needed;
    jobs.push(Job::owned(first, build_reserved(geo, first, used, plan.volume_id)));
    let fat = build_fat(geo, &plan.placed.runs);
    for i in 0..FAT_COUNT as u64 {
        let lba = first + RESERVED_SECTORS as u64 + i * geo.fat_sectors as u64;
        jobs.push(Job::owned(lba, fat.clone()));
    }
    for (i, run) in plan.placed.runs.iter().enumerate() {
        if run.clusters == 0 {
            continue;
        }
        let lba = plan.run_lba(run.first_cluster);
        let run_bytes = run.clusters as usize * geo.cluster_bytes();
        match &run.content {
            Content::Dir(_) => {
                let mut bytes = encode_dir(i, &plan.placed.runs).unwrap_or_default();
                bytes.resize(run_bytes, 0);
                jobs.push(Job::owned(lba, bytes));
            }
            Content::File(data) => {
                let whole = data.len() / SECTOR_SIZE * SECTOR_SIZE;
                if whole > 0 {
                    jobs.push(Job::borrowed(lba, &data[..whole]));
                }
                // A file that fills its clusters exactly has no tail, and a
                // driver refuses an empty write.
                if run_bytes > whole {
                    let mut tail = alloc::vec![0u8; run_bytes - whole];
                    tail[..data.len() - whole].copy_from_slice(&data[whole..]);
                    jobs.push(Job::owned(lba + (whole / SECTOR_SIZE) as u64, tail));
                }
            }
        }
    }
    jobs
}
