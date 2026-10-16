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
//! Whether the controller has written the TRB at the dequeue pointer. The
//! TRB is the controller's word in our memory, and its cycle bit is the
//! last of it the controller writes (xHCI 1.2 section 4.9.4, as recalled,
//! unverified): the control dword alone is read first, then a barrier, so
//! no other field is read before the whole TRB is seen written.

use core::sync::atomic::{fence, Ordering};

use super::state::EventRing;
use crate::constants::TRB_BYTES;
use crate::trb::Trb;

/// The control dword, holding the cycle bit, is the TRB's fourth.
const CONTROL_AT: u64 = 12;

impl EventRing {
    pub fn has_event(&self) -> bool {
        let at = self.dequeue_index as u64 * TRB_BYTES as u64 + CONTROL_AT;
        // SAFETY: the dequeue index stays inside the segment (advance.rs),
        // and the segment is this ring's own DMA grant.
        let control =
            unsafe { core::ptr::read_volatile((self.segment.user_va() + at) as *const u32) };
        let written = Trb { d3: control, ..Trb::zero() }.get_cycle() == (self.consumer_cycle != 0);
        if written {
            fence(Ordering::Acquire);
        }
        written
    }
}
