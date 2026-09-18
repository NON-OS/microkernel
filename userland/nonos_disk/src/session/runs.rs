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

//! Where a plan puts things, in sectors: the data area, each run, and the
//! file runs the receipt and the read-back use.

use alloc::vec::Vec;

use super::plan::Plan;
use crate::fat32::{data_lba, Content};
use crate::writer::FileRun;

impl<'a> Plan<'a> {
    /// The first sector of the data area, where cluster 2 begins.
    pub fn data_lba(&self) -> u64 {
        data_lba(self.layout.esp_first_lba, &self.geometry)
    }

    pub fn run_lba(&self, first_cluster: u32) -> u64 {
        self.data_lba() + (first_cluster as u64 - 2) * self.geometry.sectors_per_cluster as u64
    }

    /// Where each file's bytes land, for the receipt and the read-back.
    pub fn file_runs(&self) -> Vec<FileRun<'a>> {
        let mut files = Vec::new();
        for run in &self.placed.runs {
            if let (Content::File(data), true) = (&run.content, run.clusters > 0) {
                files.push(FileRun { lba: self.run_lba(run.first_cluster), data });
            }
        }
        files
    }
}
