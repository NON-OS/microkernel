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

//! Block Size, Transfer Mode and Command register fields.

// Block Size: transfer block size in 11:0, SDMA buffer boundary in 14:12.
// Linux writes SDHCI_MAKE_BLKSZ(7, 512): boundary 512 KiB, unused by ADMA2.
pub const BLOCK_SIZE_512: u16 = (7 << 12) | 512;

// Transfer Mode.
pub const TM_DMA: u16 = 1 << 0;
pub const TM_BLOCK_COUNT: u16 = 1 << 1;
pub const TM_AUTO_CMD12: u16 = 1 << 2;
pub const TM_READ: u16 = 1 << 4;
pub const TM_MULTI: u16 = 1 << 5;

// Command.
pub const CMD_RESP_NONE: u16 = 0;
pub const CMD_RESP_136: u16 = 1;
pub const CMD_RESP_48: u16 = 2;
pub const CMD_RESP_48_BUSY: u16 = 3;
pub const CMD_CRC_CHECK: u16 = 1 << 3;
pub const CMD_INDEX_CHECK: u16 = 1 << 4;
pub const CMD_DATA: u16 = 1 << 5;
pub const CMD_TYPE_ABORT: u16 = 3 << 6;
