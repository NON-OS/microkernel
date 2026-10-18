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

//! Finding the event ring as the controller does, from the segment table.

use nonos_libc::dma_host;

use super::producer::{reg64, Producer, TRB};
use crate::constants::{ERSTBA_LO, ERSTSZ};
use crate::trb::read_volatile_at;

impl Producer {
    /// Read the segment table the driver programmed, as the controller does
    /// when ERSTBA is written: one entry, naming a 64-byte aligned segment
    /// in memory the driver mapped for DMA.
    pub fn start(&mut self) {
        let entries = self.regs.wrote32(ERSTSZ as usize) & 0xFFFF;
        assert_eq!(entries, 1, "ERSTSZ names one segment");
        let erst = dma_host(reg64(&self.regs, ERSTBA_LO)).expect("ERSTBA names DMA memory");
        // SAFETY: `dma_host` named live host memory behind ERSTBA, the
        // driver's 16-byte segment table entry.
        let entry = unsafe { read_volatile_at(erst) };
        let base = entry.d0 as u64 | (entry.d1 as u64) << 32;
        assert_eq!(base & 0x3F, 0, "a segment base is 64-byte aligned");
        let trbs = (entry.d2 & 0xFFFF) as usize;
        assert!((16..=4096).contains(&trbs), "segment size {trbs} is outside 16..=4096");
        let last = base + (trbs as u64 - 1) * TRB;
        assert!(dma_host(last).is_some(), "the whole segment is DMA memory");
        self.segment_bus = base;
        self.segment_host = dma_host(base).expect("the segment base is DMA memory");
        self.trbs = trbs;
    }

    /// Start over on the segment table the driver programmed after a
    /// reset, as the reset controller does: at its first TRB, cycle one.
    pub fn restart(&mut self) {
        self.start();
        self.enqueue = 0;
        self.cycle = true;
    }
}
