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

//! Taking the frame at `head` once the part has given its descriptor back.
//! A frame that is chained, errored, a runt or oversized comes back with
//! length 0, so the caller still recycles the slot.

use core::ptr::{addr_of, read_volatile};
use core::sync::atomic::{fence, Ordering};

use crate::constants::queue::{ENP, ERR, MCNT_MASK, OWN, RX_DESC_COUNT, STP};
use crate::constants::{FCS_LEN, MAX_ETHERNET_FRAME, MIN_ETHERNET_FRAME};

use super::rx::RxRing;

// SAFETY anchor: `head` is held below RX_DESC_COUNT, see rx.rs.
impl RxRing {
    /// The slot and the frame length in it, or `None` while the part still
    /// owns the next descriptor.
    pub fn consume(&mut self) -> Option<(u16, u16)> {
        let d = unsafe { self.descriptor(self.head) };
        // Written by DMA: every read goes to memory, or a hoisted read never
        // sees the part let go.
        let status = unsafe { read_volatile(addr_of!((*d).status)) };
        if status & OWN != 0 {
            return None;
        }
        // The length and the frame are the driver's only once OWN is clear.
        fence(Ordering::Acquire);
        let mcnt = unsafe { read_volatile(addr_of!((*d).misc)) } & MCNT_MASK;
        let idx = self.head;
        self.head = (self.head + 1) % RX_DESC_COUNT as u16;
        Some((idx, frame_len(status, mcnt)))
    }
}

/// The frame length a finished descriptor holds, without its FCS, or 0 for
/// one the driver must drop. A frame spread over descriptors cannot happen
/// with buffers this size, so one that claims to is a part misbehaving.
pub const fn frame_len(status: u16, mcnt: u32) -> u16 {
    let whole = status & (STP | ENP) == (STP | ENP);
    let len = (mcnt as usize).saturating_sub(FCS_LEN);
    if status & ERR != 0 || !whole || len < MIN_ETHERNET_FRAME || len > MAX_ETHERNET_FRAME {
        return 0;
    }
    len as u16
}
