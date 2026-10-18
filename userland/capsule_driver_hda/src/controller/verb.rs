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

//! The verb transport: one command out through the CORB, its answer back
//! through the RIRB.
//!
//! The RIRB also carries unsolicited responses (a jack plugged in, once a pin
//! is told to report), so the two rings advance independently. The driver
//! keeps its own RIRB read pointer, as Linux does (`snd_hdac_bus_update_rirb`),
//! consumes every entry the controller has written, sets unsolicited ones
//! aside by the UNSOL bit of the entry's upper word, and takes the first
//! solicited answer from the codec that was asked (`rirb`). A command that
//! gets no answer still moved the controller's write pointer, so the shadow
//! moves with it, or every verb after one slow answer would time out.

use crate::regs::Regs;

/// Linux waits up to a second for an answer; a working codec answers in
/// microseconds, and a setup that waits a second per verb on a dead one
/// takes minutes to give up. 100 ms covers the slowest power transition a
/// codec acknowledges before it completes.
pub const VERB_MS: u64 = 100;

pub struct Link {
    pub(super) regs: Regs,
    pub(super) corb_va: u64,
    pub(super) rirb_va: u64,
    pub(super) mask: u16,
    pub(super) wp: u16,
    pub(super) rp: u16,
    pub(super) unsolicited: u32,
}

impl Link {
    /// A link over rings of `entries` slots, both pointers at zero, as
    /// `corb::init` leaves them.
    pub fn new(regs: Regs, corb_va: u64, rirb_va: u64, entries: u16) -> Self {
        let mask = if entries == 0 { 0xff } else { entries - 1 };
        Link { regs, corb_va, rirb_va, mask, wp: 0, rp: 0, unsolicited: 0 }
    }

    /// The controller register window, for the Immediate Command fallback when
    /// a verb gets no answer over the ring.
    pub fn regs(&self) -> Regs {
        self.regs
    }

    /// Unsolicited responses seen and set aside. A codec sends one when a
    /// pin's jack changes, so a new one is a reason to read the jack now.
    pub fn unsolicited(&self) -> u32 {
        self.unsolicited
    }
}
