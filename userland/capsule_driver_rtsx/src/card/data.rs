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

//! The command buffer entries a data phase takes, read or write
//! (sd_cmd_set_data_len and the DMA setup of sd_read_long_data and
//! sd_write_long_data): block count and size, the DMA-done flag cleared,
//! the transfer count, the DMA direction, and the ring buffer as the data
//! source.

use crate::regs::card::{CARD_DATA_SOURCE, RING_BUFFER};
use crate::regs::pm::*;
use crate::regs::sd::{SD_BLOCK_CNT_H, SD_BLOCK_CNT_L, SD_BYTE_CNT_H, SD_BYTE_CNT_L};
use crate::wire::CmdBuf;

/// `count` blocks of 512 bytes, to the card when `to_card`.
pub fn data_phase(buf: &mut CmdBuf, count: u16, to_card: bool) {
    let blocks = count.to_le_bytes();
    buf.write(SD_BLOCK_CNT_L, 0xFF, blocks[0]).write(SD_BLOCK_CNT_H, 0xFF, blocks[1]);
    buf.write(SD_BYTE_CNT_L, 0xFF, 0x00).write(SD_BYTE_CNT_H, 0xFF, 0x02);
    buf.write(IRQSTAT0, DMA_DONE_INT, DMA_DONE_INT);
    let tc = (count as u32 * 512).to_le_bytes();
    buf.write(DMATC3, 0xFF, tc[3]).write(DMATC2, 0xFF, tc[2]);
    buf.write(DMATC1, 0xFF, tc[1]).write(DMATC0, 0xFF, tc[0]);
    let dir = if to_card { DMA_DIR_TO_CARD } else { DMA_DIR_FROM_CARD };
    let mask = DMA_EN | DMA_DIR_FROM_CARD | DMA_PACK_SIZE_MASK;
    buf.write(DMACTL, mask, DMA_EN | dir | DMA_512);
    buf.write(CARD_DATA_SOURCE, 0x01, RING_BUFFER);
}
