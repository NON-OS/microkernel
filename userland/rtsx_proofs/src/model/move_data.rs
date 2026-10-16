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

//! The bytes a data phase moves: the length the driver programmed, and the
//! card's blocks to or from memory at the scatter-gather entry's address.

use nonos_libc::{dma_read, dma_write};

use super::reader::Reader;
use crate::regs::pm::{DMATC0, DMATC1, DMATC2, DMATC3};
use crate::regs::sd::{SD_BLOCK_CNT_H, SD_BLOCK_CNT_L, SD_BYTE_CNT_H, SD_BYTE_CNT_L};

impl Reader {
    pub(super) fn planned_len(&self) -> usize {
        let half = |lo, hi| u16::from_le_bytes([self.reg(lo), self.reg(hi)]) as usize;
        let (blocks, size) =
            (half(SD_BLOCK_CNT_L, SD_BLOCK_CNT_H), half(SD_BYTE_CNT_L, SD_BYTE_CNT_H));
        let tc = [DMATC0, DMATC1, DMATC2, DMATC3].map(|r| self.reg(r));
        let count = u32::from_le_bytes(tc) as usize;
        if blocks * size == count {
            count
        } else {
            usize::MAX
        }
    }

    pub(super) fn card_to_host(&mut self, addr: u64, len: usize) -> bool {
        let out = self.slot.as_mut().and_then(|c| c.read_out(len));
        out.is_some_and(|bytes| dma_write(addr, &bytes))
    }

    pub(super) fn host_to_card(&mut self, addr: u64, len: usize) -> bool {
        let Some(bytes) = dma_read(addr, len) else { return false };
        self.slot.as_mut().is_some_and(|c| c.write_in(&bytes))
    }
}
