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

//! What an install produced, in the terms a person and a verifier both
//! need: the identifiers a firmware menu or a partition tool will show, the
//! layout and geometry that were chosen, where every file's bytes landed,
//! and every write that stays on the disk, for the read-back.

use alloc::vec::Vec;

use crate::fat32::Geometry;
use crate::guid::Guid;
use crate::layout::Layout;
use crate::session::Job;

/// One file's bytes on the ESP and the first sector holding them. The data
/// is the source slice the install was given.
#[derive(Debug, Clone, Copy)]
pub struct FileRun<'a> {
    pub lba: u64,
    pub data: &'a [u8],
}

#[derive(Debug)]
pub struct Receipt<'a> {
    pub disk_guid: Guid,
    /// Each partition's unique GUID, in `Region::ALL` order.
    pub partitions: [Guid; 4],
    pub layout: Layout,
    pub geometry: Geometry,
    pub bytes_written: u64,
    /// Files in the store the install wrote.
    pub store_files: usize,
    pub files: Vec<FileRun<'a>>,
    pub(crate) written: Vec<Job<'a>>,
}

impl Receipt<'_> {
    /// The ESP's unique GUID, the one a firmware boot entry names.
    pub fn esp_guid(&self) -> Guid {
        self.partitions[3]
    }
}
