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

//! Receive completions. Each names the ring 0 slot the device filled; the
//! device fills them in order, so it must be the one after the descriptor
//! held back, or the completion is not one this driver can trust.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use super::desc::completed;
use super::rx::RxRing;
use crate::constants::ring::{
    COMP_IDX_MASK, LEN_MASK, RCD_EOP, RCD_ERR, RCD_RQID_MASK, RCD_RQID_SHIFT, RCD_SOP, RX_COMP,
    RX_RING,
};
use crate::constants::{MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Completion {
    /// A whole frame in slot `idx`; post one descriptor once it is copied.
    Frame { idx: u16, len: u16 },
    /// A ring 0 slot used for a frame that is dropped; post one descriptor.
    Dropped,
    /// Not a slot this driver handed over; post nothing.
    Stray,
}

// SAFETY anchor: see rx.rs.
impl RxRing {
    pub fn consume(&mut self) -> Option<Completion> {
        let c = unsafe { self.comp(self.comp_next) };
        if !unsafe { completed(c, self.comp_gen) } {
            return None;
        }
        // The words and the frame are the driver's only once the gen is seen.
        fence(Ordering::Acquire);
        let (d0, d2) =
            unsafe { (read_volatile(addr_of!((*c).d0)), read_volatile(addr_of!((*c).d2))) };
        self.comp_next = (self.comp_next + 1) % RX_COMP as u16;
        if self.comp_next == 0 {
            self.comp_gen ^= 1;
        }
        Some(classify(d0, d2, (self.fill + 1) % RX_RING as u16))
    }
}

/// What a completion means, given the slot the device must fill next.
pub const fn classify(d0: u32, d2: u32, expected: u16) -> Completion {
    let idx = d0 & COMP_IDX_MASK;
    let rq = (d0 >> RCD_RQID_SHIFT) & RCD_RQID_MASK;
    if rq != 0 || idx != expected as u32 {
        return Completion::Stray;
    }
    let len = (d2 & LEN_MASK) as usize;
    let whole = d0 & (RCD_SOP | RCD_EOP) == RCD_SOP | RCD_EOP;
    if !whole || d2 & RCD_ERR != 0 || len < MIN_ETHERNET_FRAME || len > MAX_ETHERNET_FRAME {
        return Completion::Dropped;
    }
    Completion::Frame { idx: idx as u16, len: len as u16 }
}
