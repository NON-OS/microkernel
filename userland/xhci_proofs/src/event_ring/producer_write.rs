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

//! Writing events as the controller does: each with the producer cycle,
//! one still being written, and poison past the segment.

use super::producer::{Producer, TRB};
use crate::trb::{write_volatile_at, Trb};

/// What sits past the segment in the same page: a TRB no event ring holds.
pub const POISON: u64 = 0xBAD0_0000_0000_BAD0;

impl Producer {
    /// Write `event` at the enqueue slot with the producer cycle and move on.
    pub fn post(&mut self, mut event: Trb) {
        assert!(self.room() > 0, "the event ring is full: the driver has not handed events back");
        event.set_cycle(self.cycle);
        // SAFETY: `slot_host` is a TRB slot of the driver's live segment.
        unsafe { write_volatile_at(self.slot_host(self.enqueue), event) };
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
        // SAFETY: as in `post`.
        unsafe { write_volatile_at(self.slot_host(self.enqueue), event) };
    }

    /// Fill the rest of the segment's page with TRBs that look like events
    /// under either cycle state, so a read past the segment shows up.
    pub fn poison_past_segment(&self) {
        let page_end = (self.trbs as u64 * TRB).next_multiple_of(4096) / TRB;
        for at in self.trbs..page_end as usize {
            let mut t = Trb { d0: POISON as u32, d1: (POISON >> 32) as u32, d2: 0, d3: 63 << 10 };
            t.set_cycle(at % 2 == 0);
            // SAFETY: as in `post`.
            unsafe { write_volatile_at(self.slot_host(at), t) };
        }
    }
}

pub fn is_poison(t: &Trb) -> bool {
    t.get_pointer() == POISON
}
