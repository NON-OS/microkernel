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

//! Sizes the package store is read and written in, and where it ends.

pub const SECTOR_SIZE: usize = 512;
/// The kernel's store read takes at most sixty-four sectors a call.
pub const MAX_SECTORS_PER_REQUEST: usize = 64;
pub const MAX_READ_BYTES: usize = SECTOR_SIZE * MAX_SECTORS_PER_REQUEST;
/// The disk plan's sector. The kernel reads and writes the store only below
/// it, so the store ends there whatever the size of the disk.
pub const STORE_END_LBA: u64 = 65_536;
