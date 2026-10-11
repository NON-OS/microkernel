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

//! The three DMA grants in host memory, and the rings and status block the
//! driver keeps over them.

use crate::constants::layout::*;
use crate::queue::{RxQueue, Status, TxRing};

pub const RINGS_PHYS: u64 = 0x1_0000_0000;
pub const RX_BUF_PHYS: u64 = 0x1_0010_0000;
pub const TX_BUF_PHYS: u64 = 0x1_0020_0000;

/// Eight-byte cells, so every BD and the status block are aligned.
pub struct Memory {
    pub rings: Box<[u64]>,
    pub rx: Box<[u8]>,
    pub tx: Box<[u8]>,
}

impl Memory {
    pub fn new() -> Self {
        Self {
            rings: vec![0u64; RINGS_BYTES / 8].into_boxed_slice(),
            rx: vec![0u8; RX_BUFFER_POOL_BYTES].into_boxed_slice(),
            tx: vec![0u8; TX_BUFFER_POOL_BYTES].into_boxed_slice(),
        }
    }

    /// A 32-bit word of the rings grant, as the part would write it.
    pub fn put32(&mut self, at: usize, v: u32) {
        let w = &mut self.rings[at / 8];
        let shift = (at % 8) * 8;
        *w = (*w & !(0xFFFF_FFFFu64 << shift)) | (v as u64) << shift;
    }

    pub fn get32(&self, at: usize) -> u32 {
        (self.rings[at / 8] >> ((at % 8) * 8)) as u32
    }

    pub fn rings_va(&mut self) -> u64 {
        self.rings.as_mut_ptr() as u64
    }

    pub fn rx_queue(&mut self) -> RxQueue {
        let va = self.rings_va();
        RxQueue::new(va, RINGS_PHYS, self.rx.as_mut_ptr() as u64, RX_BUF_PHYS)
    }

    pub fn tx_ring(&mut self) -> TxRing {
        let va = self.rings_va();
        TxRing::new(va, RINGS_PHYS, self.tx.as_mut_ptr() as u64, TX_BUF_PHYS)
    }

    pub fn status(&mut self) -> Status {
        Status::new(self.rings_va() + STATUS_AT as u64)
    }
}
