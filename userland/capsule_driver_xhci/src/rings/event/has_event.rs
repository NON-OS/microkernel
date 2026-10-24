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

use super::state::EventRing;
use crate::constants::{TRB_BYTES, TRB_CYCLE};
use crate::dma::read_barrier;
/// Dword 3 of a TRB holds its cycle bit.
const CYCLE_DWORD: u64 = 12;
impl EventRing {
    /// Whether the controller has written the TRB at the dequeue pointer:
    /// its cycle bit, which the controller writes last, is this lap's.
    /// Only that dword is read, and the barrier after it keeps the rest of
    /// the event, read next (`current_trb`), from being read before it: a
    /// weakly ordered CPU could otherwise pair the new cycle bit with the
    /// slot's previous event.
    pub fn has_event(&self) -> bool {
        let at = self.dequeue_index as u64 * TRB_BYTES as u64 + CYCLE_DWORD;
        // SAFETY: `dequeue_index` is below EVENT_RING_SEGMENT_TRBS (`advance`
        // wraps it), so the dword lies inside the live segment grant.
        let d3 = unsafe { core::ptr::read_volatile((self.segment.user_va() + at) as *const u32) };
        let written = (d3 & TRB_CYCLE != 0) == (self.consumer_cycle != 0);
        if written {
            read_barrier();
        }
        written
    }
}
