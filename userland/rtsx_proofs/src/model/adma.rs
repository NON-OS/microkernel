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

//! ADMA: one scatter-gather entry at HDBAR, the data phase the SD engine
//! armed moved through it, then the command buffer waiting at its check
//! resumed. The entry must be valid, last and a data entry, and its length
//! must be what DMATC and SD_BLOCK_CNT times SD_BYTE_CNT say; otherwise the
//! transfer ends in an error. A stop halts it; only a reset disarms it.

use nonos_libc::{dma_read, note};

use super::knobs::Phase;
use super::linux::{DMA_RST, HDBCTLR_TO_HOST};
use super::reader::Reader;
use crate::regs::pm::DMACTL;

/// Valid, end of table, and a data transfer (rtsx_pci_add_sg_tbl).
const SG_DATA_END: u64 = 0x01 | 0x02 | 0x20;

impl Reader {
    pub(super) fn adma(&mut self, v: u32) {
        self.dma_stopped = false;
        if self.knobs.pull_on_data {
            self.pull();
            return;
        }
        let Some(phase) = self.data.filter(|_| !self.knobs.stall_data) else { return };
        let Some(sg) = dma_read(self.hdbar as u64, 8) else { return };
        let entry = sg.iter().rev().fold(0u64, |acc, &b| acc << 8 | b as u64);
        let (addr, len) = (entry >> 32, ((entry >> 12) & 0xF_FFFF) as usize);
        let to_host = v & HDBCTLR_TO_HOST != 0;
        let fits = entry & 0x3F == SG_DATA_END && len == self.planned_len();
        let moved = fits
            && match phase {
                Phase::Read => to_host && self.card_to_host(addr, len),
                Phase::Write => !to_host && self.host_to_card(addr, len),
            };
        self.end_data(moved && !self.knobs.data_error);
        if let Some(run) = self.run.take() {
            self.resume(run);
        }
    }

    /// STOP_DMA halts a transfer; the engine stays armed at its entry
    /// until the DMA reset that follows (rtsx_pci_stop_cmd writes both).
    pub(super) fn stop_dma(&mut self) {
        note("stop dma");
        self.dma_stopped = true;
    }

    /// DMACTL's reset bit, which clears itself once taken.
    pub(super) fn reset_dma(&mut self) {
        note("dma reset");
        self.regs.insert(DMACTL, self.reg(DMACTL) & !DMA_RST);
        if !self.knobs.late {
            self.data = None;
        }
    }
}
