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

//! One internal register through HAIMR, and what a write to some of them
//! sets going: the PHY's busy bit clears at once, a start bit in
//! SD_TRANSFER runs the SD engine, the slot's power follows CARD_PWR_CTL,
//! a DMA reset disarms ADMA (adma.rs), and the ring-buffer flush is noted
//! for teardown order. A register a test leaves untaken keeps its value,
//! and HAIMR answers that, not the data written.

use nonos_libc::note;

use super::linux::{DMA_RST, RB_FLUSH};
use super::reader::Reader;
use crate::regs::card::{CARD_PWR_CTL, SD_POWER_MASK, SD_POWER_OFF};
use crate::regs::host::{HAIMR_TRANS_START, HAIMR_WRITE};
use crate::regs::phy::{PHYRWCTL, PHY_BUSY};
use crate::regs::pm::{DMACTL, RBCTL};
use crate::regs::sd::{SD_TRANSFER, SD_TRANSFER_START};

impl Reader {
    /// The value HAIMR reads back once the access is done: the low byte is
    /// the data a write took, or the register a read asked for.
    pub(super) fn access(&mut self, v: u32) -> u32 {
        let reg = 0xC000 | ((v >> 16) & 0x3FFF) as u16;
        self.accesses += 1;
        let low = if v & HAIMR_WRITE != 0 && self.knobs.untaken != Some(reg) {
            self.write_reg(reg, (v >> 8) as u8, v as u8);
            v as u8
        } else {
            self.reg(reg)
        };
        (v & !HAIMR_TRANS_START & 0xFFFF_FF00) | low as u32
    }

    pub fn reg(&self, reg: u16) -> u8 {
        self.regs.get(&reg).copied().unwrap_or(0)
    }

    pub(super) fn write_reg(&mut self, reg: u16, mask: u8, data: u8) {
        let new = (self.reg(reg) & !mask) | (data & mask);
        self.regs.insert(reg, new);
        match reg {
            PHYRWCTL => {
                self.regs.insert(reg, new & !PHY_BUSY);
            }
            SD_TRANSFER if new & SD_TRANSFER_START != 0 => self.sd_transfer(new),
            CARD_PWR_CTL if new & SD_POWER_MASK == SD_POWER_OFF => self.power_off_slot(),
            DMACTL if data & mask & DMA_RST != 0 => self.reset_dma(),
            RBCTL if data & mask & RB_FLUSH != 0 => note("ring buffer flush"),
            _ => {}
        }
    }
}
