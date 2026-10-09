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

//! Reading blocks by DMA (sd_read_long_data and rtsx_pci_dma_transfer):
//! the command and transfer size into the command buffer, the DMA engine
//! pointed at the data buffer through one scatter-gather entry, an auto
//! read transfer, and CMD12 after a multi-block read.

use super::command::{put_command, send_command};
use super::info::Card;
use super::select::select_sd;
use crate::engine::{clear_error, start, start_read, wait};
use crate::error::{Result, RtsxError};
use crate::regs::card::{CARD_DATA_SOURCE, RING_BUFFER};
use crate::regs::pm::*;
use crate::regs::sd::*;
use crate::sd::Command;
use crate::setup::Driver;
use crate::wire::{CmdBuf, CmdKind};

/// What the data buffer holds: 128 blocks of 512 bytes.
pub const MAX_BLOCKS: u16 = 128;

/// `count` blocks from `lba` into the data buffer.
pub fn read_blocks(drv: &Driver, card: &Card, lba: u32, count: u16) -> Result<()> {
    let count = count.clamp(1, MAX_BLOCKS);
    let len = count as u32 * 512;
    let addr = if card.high_capacity { lba } else { lba.saturating_mul(512) };
    let multi = count > 1;
    let cmd = if multi { Command::read_multiple(addr) } else { Command::read_single(addr) };
    let mut buf = CmdBuf::new();
    select_sd(&mut buf);
    put_command(&mut buf, cmd);
    buf.write(SD_BLOCK_CNT_L, 0xFF, count as u8).write(SD_BLOCK_CNT_H, 0xFF, (count >> 8) as u8);
    buf.write(SD_BYTE_CNT_L, 0xFF, 0x00).write(SD_BYTE_CNT_H, 0xFF, 0x02);
    buf.write(IRQSTAT0, DMA_DONE_INT, DMA_DONE_INT);
    let tc = len.to_le_bytes();
    buf.write(DMATC3, 0xFF, tc[3]).write(DMATC2, 0xFF, tc[2]);
    buf.write(DMATC1, 0xFF, tc[1]).write(DMATC0, 0xFF, tc[0]);
    buf.write(DMACTL, 0x03 | DMA_PACK_SIZE_MASK, DMA_DIR_FROM_CARD | DMA_EN | DMA_512);
    buf.write(CARD_DATA_SOURCE, 0x01, RING_BUFFER);
    // Not a UHS card: the write CRC timeout check is off, as Linux has it.
    buf.write(SD_CFG2, 0xFF, SD_NO_CHECK_WAIT_CRC_TO | cmd.rsp.cfg2());
    buf.write(SD_TRANSFER, 0xFF, SD_TRANSFER_START | SD_TM_AUTO_READ_2);
    buf.add(CmdKind::Check, SD_TRANSFER, SD_TRANSFER_END, SD_TRANSFER_END);
    start(drv, &buf)?;
    start_read(drv, len);
    let done = wait(drv, 10_000, RtsxError::DataTimeout, RtsxError::DataFailed);
    if done.is_err() {
        clear_error(drv);
    }
    if multi {
        let stop = send_command(drv, Command::STOP_TRANSMISSION);
        return done.and(stop.map(|_| ()));
    }
    done
}
