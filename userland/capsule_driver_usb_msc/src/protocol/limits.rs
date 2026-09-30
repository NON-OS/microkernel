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

pub const STATUS_LEN: usize = 4;
pub const MAX_BINDINGS: usize = 8;
pub const CBW_LEN: usize = 31;
pub const CSW_LEN: usize = 13;
pub const BLOCK_BYTES: u32 = 512;
pub const MAX_TRANSFER_BLOCKS: u16 = 128;
/// A block request: `lba_le64, sectors_le32`, then for a write the data.
pub const BLK_HEADER_LEN: usize = 12;
/// Sectors one block request moves, as the kernel client caps them.
pub const BLK_MAX_SECTORS: u32 = 64;
pub const BLK_MAX_BYTES: usize = BLK_MAX_SECTORS as usize * BLOCK_BYTES as usize;
/// The kernel client's reply inbox, `endpoint.4294967315`.
pub const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_0013;
