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

//! The block surface a server calls: capacity, read, write, flush, and the
//! descriptions the info ops reply with. Sectors are 512 bytes; one request
//! moves at most MAX_SECTORS of them, the AHCI capsule's limit, through the
//! 32 KiB data buffer.

mod block_io;
mod describe;
mod emmc_disk;
mod lifecycle;
mod sizes;

pub use emmc_disk::EmmcDisk;
pub use sizes::{DATA_BUF_BYTES, MAX_SECTORS, SECTOR_SIZE};
