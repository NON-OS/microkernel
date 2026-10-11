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

//! The one DMA grant: the descriptor table in the first page, the transfer
//! buffer after it. Every transfer moves through the buffer, so it bounds
//! one request.

use super::ata::SECTOR_BYTES;

pub const PRD_OFFSET: u64 = 0;
/// Room for the table: one descriptor per 64 KiB boundary the buffer
/// crosses, and one more.
pub const PRD_TABLE_BYTES: usize = 64;
pub const DATA_OFFSET: u64 = 0x1000;
/// 128 sectors, the largest request served.
pub const MAX_SECTORS: u32 = 128;
pub const DATA_BYTES: usize = MAX_SECTORS as usize * SECTOR_BYTES;
pub const GRANT_BYTES: u64 = DATA_OFFSET + DATA_BYTES as u64;
