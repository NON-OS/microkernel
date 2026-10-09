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

//! eMMC command indices and arguments (JEDEC eMMC 5.1, 6.10.4).

pub const GO_IDLE_STATE: u8 = 0;
pub const SEND_OP_COND: u8 = 1;
pub const ALL_SEND_CID: u8 = 2;
pub const SET_RELATIVE_ADDR: u8 = 3;
pub const SWITCH: u8 = 6;
pub const SELECT_CARD: u8 = 7;
pub const SEND_EXT_CSD: u8 = 8;
pub const SEND_CSD: u8 = 9;
pub const STOP_TRANSMISSION: u8 = 12;
pub const SEND_STATUS: u8 = 13;
pub const SET_BLOCKLEN: u8 = 16;
pub const READ_SINGLE_BLOCK: u8 = 17;
pub const READ_MULTIPLE_BLOCK: u8 = 18;
pub const SET_BLOCK_COUNT: u8 = 23;
pub const WRITE_BLOCK: u8 = 24;
pub const WRITE_MULTIPLE_BLOCK: u8 = 25;

/// The relative address the driver gives the card, as Linux does.
pub const RCA: u16 = 1;

pub const fn rca_arg(rca: u16) -> u32 {
    (rca as u32) << 16
}

/// SWITCH access mode Write Byte: EXT_CSD[index] = value.
pub const SWITCH_WRITE_BYTE: u32 = 3;

/// CMD6 argument: access 25:24, index 23:16, value 15:8, command set 2:0
/// (0, the standard set).
pub const fn switch_arg(index: u8, value: u8) -> u32 {
    (SWITCH_WRITE_BYTE << 24) | ((index as u32) << 16) | ((value as u32) << 8)
}

/// CMD23 argument for `blocks` blocks: count in 15:0, Reliable Write
/// (bit 31) and the packed and tag bits clear.
pub const fn block_count_arg(blocks: u16) -> u32 {
    blocks as u32
}

/// The address argument of a read or write: the sector number on a card in
/// sector mode, the byte offset on a byte-addressed card (2 GB or less).
/// None when it does not fit 32 bits.
pub const fn data_arg(lba: u64, sector_mode: bool) -> Option<u32> {
    let a = if sector_mode {
        lba
    } else {
        match lba.checked_mul(512) {
            Some(b) => b,
            None => return None,
        }
    };
    if a > u32::MAX as u64 {
        None
    } else {
        Some(a as u32)
    }
}
