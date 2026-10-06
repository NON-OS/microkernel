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
//! at the front of the disk, a backup pair at the end, and four partitions.

mod entry;
mod header;
mod mbr;
mod names;
mod probe;
mod shape;
mod table;

pub use probe::written_by_nonos;
pub use shape::{ARRAY_SECTORS, FIRST_USABLE_LBA};
pub use table::table;
