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

//! The partition table: a protective MBR, a primary header and entry array
//! at the front of the disk, a backup pair at the end, and one partition.

mod entry;
mod header;
mod layout;
mod mbr;
mod write;

pub use entry::PARTITION_NAME;
pub use layout::{Layout, ESP_FIRST_LBA};
pub use write::write_table;
