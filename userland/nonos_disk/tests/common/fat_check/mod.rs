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

//! A FAT32 volume read by the specification, sharing no code with the
//! writer: the boot sector and FSInfo and their backups, both tables, every
//! cluster chain from the root down, every directory with its long-name
//! slots, and each file's bytes. What fsck or a firmware driver would trip
//! over panics here with what it was. mtools reads the volume as well
//! (`mtools_reads_every_file_back.rs`) where it is installed; this runs
//! everywhere.

mod boot;
pub mod entries;
mod walk;

pub use walk::{check, Volume};
