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

//! The task file for one command, worked out before any of it is written.
//! LBA28 when the whole range sits below 2^28, LBA48 otherwise, which the
//! disk must offer. Pure, for the host proofs.

use crate::constants::ata::{
    dev_select, CMD_FLUSH_CACHE, CMD_FLUSH_CACHE_EXT, CMD_READ_DMA, CMD_READ_DMA_EXT,
    CMD_WRITE_DMA, CMD_WRITE_DMA_EXT, DEV_LBA, LBA28_LIMIT, REG_COMMAND, REG_COUNT, REG_DEVICE,
    REG_LBA_HI, REG_LBA_LO, REG_LBA_MID,
};
use crate::disk::Flush;
use crate::regs::Port;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Read,
    Write,
}

/// The register writes, in the order ATA requires: for LBA48 each register
/// takes its high byte first, then its low byte.
pub struct Taskfile {
    pub writes: [(u16, u8); 11],
    pub n: usize,
}

impl Taskfile {
    /// The DMA command for `count` sectors at `lba` on `drive`, or None for a
    /// range the disk cannot address.
    pub fn dma(cmd: Command, drive: u8, lba: u64, count: u32, lba48: bool) -> Option<Self> {
        let end = lba.checked_add(count as u64)?;
        if count == 0 || count > 256 {
            return None;
        }
        if end <= LBA28_LIMIT {
            let op = match cmd {
                Command::Read => CMD_READ_DMA,
                Command::Write => CMD_WRITE_DMA,
            };
            let dev = dev_select(drive) | DEV_LBA | ((lba >> 24) as u8 & 0x0F);
            // A count of 256 is written as zero.
            return Some(Self::of(&[
                (REG_COUNT, count as u8),
                (REG_LBA_LO, lba as u8),
                (REG_LBA_MID, (lba >> 8) as u8),
                (REG_LBA_HI, (lba >> 16) as u8),
                (REG_DEVICE, dev),
                (REG_COMMAND, op),
            ]));
        }
        if !lba48 {
            return None;
        }
        let op = match cmd {
            Command::Read => CMD_READ_DMA_EXT,
            Command::Write => CMD_WRITE_DMA_EXT,
        };
        Some(Self::of(&[
            (REG_COUNT, (count >> 8) as u8),
            (REG_LBA_LO, (lba >> 24) as u8),
            (REG_LBA_MID, (lba >> 32) as u8),
            (REG_LBA_HI, (lba >> 40) as u8),
            (REG_COUNT, count as u8),
            (REG_LBA_LO, lba as u8),
            (REG_LBA_MID, (lba >> 8) as u8),
            (REG_LBA_HI, (lba >> 16) as u8),
            (REG_DEVICE, dev_select(drive) | DEV_LBA),
            (REG_COMMAND, op),
        ]))
    }

    /// The flush the disk takes, or None for one with nothing to flush.
    pub fn flush(drive: u8, flush: Flush) -> Option<Self> {
        let op = match flush {
            Flush::None => return None,
            Flush::Cache => CMD_FLUSH_CACHE,
            Flush::CacheExt => CMD_FLUSH_CACHE_EXT,
        };
        Some(Self::of(&[(REG_DEVICE, dev_select(drive)), (REG_COMMAND, op)]))
    }

    fn of(list: &[(u16, u8)]) -> Self {
        let mut writes = [(0u16, 0u8); 11];
        writes[..list.len()].copy_from_slice(list);
        Self { writes, n: list.len() }
    }

    /// Write it to the command block; the last write is the command.
    pub fn issue(&self, cmd: &Port) {
        for &(reg, v) in &self.writes[..self.n] {
            cmd.write8(reg, v);
        }
    }
}
