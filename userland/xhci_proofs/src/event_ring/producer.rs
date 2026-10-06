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

//! The controller's side of the event ring (xHCI 1.2 section 4.9.4).
//!
//! It finds the ring the way hardware does, through the segment table the
//! driver programmed into ERSTBA and ERSTSZ, and never through the driver's
//! own fields. It writes each event with its producer cycle state, flips that
//! state when it wraps at the segment size the table gives, and refuses to
//! write over an event the driver has not handed back through ERDP: the ring
//! is full when the enqueue pointer is one behind the dequeue pointer.

use nonos_devmodel::FakeBar;
use nonos_libc::dma_host;

use crate::constants::{ERDP_LO, ERSTBA_LO, ERSTSZ, TRB_BYTES};
use crate::trb::{read_volatile_at, write_volatile_at, Trb};

/// IMAN through the top of ERDP.
pub const INTERRUPTER_BYTES: usize = 0x20;
/// The low four bits of ERDP: DESI in 2:0 and EHB in 3.
pub const ERDP_FLAGS: u64 = 0xF;
pub const ERDP_EHB_BIT: u64 = 1 << 3;
/// What sits past the segment in the same page: a TRB no event ring holds.
pub const POISON: u64 = 0xBAD0_0000_0000_BAD0;

const TRB: u64 = TRB_BYTES as u64;

pub fn reg64(bar: &FakeBar, off: u64) -> u64 {
    let off = off as usize;
    bar.wrote32(off) as u64 | (bar.wrote32(off + 4) as u64) << 32
}

pub struct Producer {
    pub regs: FakeBar,
    segment_bus: u64,
    segment_host: u64,
    trbs: usize,
    enqueue: usize,
    cycle: bool,
}

impl Producer {
    /// The interrupter's registers, as yet unprogrammed.
    pub fn new() -> Self {
        Self {
            regs: FakeBar::new(INTERRUPTER_BYTES),
            segment_bus: 0,
            segment_host: 0,
            trbs: 0,
            enqueue: 0,
            cycle: true,
        }
    }

    /// Read the segment table the driver programmed, as the controller does
    /// when ERSTBA is written: one entry, naming a 64-byte aligned segment
    /// in memory the driver mapped for DMA.
    pub fn start(&mut self) {
        let entries = self.regs.wrote32(ERSTSZ as usize) & 0xFFFF;
        assert_eq!(entries, 1, "ERSTSZ names one segment");
        let erst = dma_host(reg64(&self.regs, ERSTBA_LO)).expect("ERSTBA names DMA memory");
        let entry = read_volatile_at(erst);
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

    pub fn segment_trbs(&self) -> usize {
        self.trbs
    }

    pub fn segment_bus(&self) -> core::ops::Range<u64> {
        self.segment_bus..self.segment_bus + self.trbs as u64 * TRB
    }

    pub fn erdp(&self) -> u64 {
        reg64(&self.regs, ERDP_LO)
    }

    /// The slot ERDP names. A pointer outside the segment fails here: the
    /// controller would have nowhere to take it.
    pub fn dequeue(&self) -> usize {
        let at = self.erdp() & !ERDP_FLAGS;
        assert!(self.segment_bus().contains(&at), "ERDP {at:#x} is outside the segment");
        ((at - self.segment_bus) / TRB) as usize
    }

    /// Events written and not yet handed back.
    pub fn pending(&self) -> usize {
        (self.enqueue + self.trbs - self.dequeue()) % self.trbs
    }

    /// Room for more events before the ring is full.
    pub fn room(&self) -> usize {
        self.trbs - 1 - self.pending()
    }

    /// Write `event` at the enqueue slot with the producer cycle and move on.
    pub fn post(&mut self, mut event: Trb) {
        assert!(self.room() > 0, "the event ring is full: the driver has not handed events back");
        event.set_cycle(self.cycle);
        write_volatile_at(self.slot_host(self.enqueue), event);
        self.enqueue += 1;
        if self.enqueue == self.trbs {
            self.enqueue = 0;
            self.cycle = !self.cycle;
        }
    }

    /// Write `event` at the enqueue slot with the cycle bit the driver is not
    /// looking for, and stay there: an event still being written. The next
    /// `post` overwrites it.
    pub fn scribble(&mut self, mut event: Trb) {
        event.set_cycle(!self.cycle);
        write_volatile_at(self.slot_host(self.enqueue), event);
    }

    /// Fill the rest of the segment's page with TRBs that look like events
    /// under either cycle state, so a read past the segment shows up.
    pub fn poison_past_segment(&self) {
        let page_end = (self.trbs as u64 * TRB).next_multiple_of(4096) / TRB;
        for at in self.trbs..page_end as usize {
            let mut t = Trb { d0: POISON as u32, d1: (POISON >> 32) as u32, d2: 0, d3: 63 << 10 };
            t.set_cycle(at % 2 == 0);
            write_volatile_at(self.slot_host(at), t);
        }
    }

    fn slot_host(&self, at: usize) -> u64 {
        self.segment_host + at as u64 * TRB
    }
}

pub fn is_poison(t: &Trb) -> bool {
    t.get_pointer() == POISON
}
