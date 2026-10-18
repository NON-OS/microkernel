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

//! The used ring, which the device writes: its index and element are the
//! device's word, taken only when they name the one request in flight.

use super::chain::HEAD;
use super::fault::Fault;
use super::layout::Queue;
use crate::constants::STATUS_OFFSET;

/// VirtqUsed: u16 flags, u16 idx, VirtqUsedElem { u32 id, u32 len } ring[].
const USED_IDX: usize = 2;
const USED_RING: usize = 4;
const ELEM_BYTES: usize = 8;

impl Queue {
    /// The request in flight, once the device's used index has moved;
    /// `None` while it is still the device's. An index that moved by
    /// anything but the one request, or an element naming another head, is
    /// the device out of turn: the queue stalls until a reset. A length
    /// other than the chain's writable bytes fails the request, which is
    /// answered, so the queue goes on: a read the device did not fill is
    /// not served as if it had. On legacy a length that claims the status
    /// byte is enough: the specification's legacy notes have a driver
    /// ignore it, as many legacy devices (bhyve's among them) report the
    /// status byte alone. The status is believed there, since it reads 0xFF
    /// until the device writes it and a read's buffer was emptied first.
    pub fn completion(&mut self) -> Option<Result<(), Fault>> {
        let used = self.used_ring();
        let idx = self.ring.r16(used + USED_IDX);
        if idx == self.used {
            return None;
        }
        // The element, the status and the data are the device's only once
        // the index that covers them is seen (libc's barrier, which orders
        // what the device wrote, where a fence on aarch64 does not).
        nonos_libc::dma_rmb();
        let at = used + USED_RING + ELEM_BYTES * self.pos(self.used);
        let (id, len) = (self.ring.r32(at), self.ring.r32(at + 4));
        if !self.outstanding() || idx != self.posted || id != u32::from(HEAD) {
            return Some(Err(self.stall(Fault::Foreign)));
        }
        self.used = idx;
        if (self.legacy && len == 0) || (!self.legacy && len != self.expect) {
            return Some(Err(Fault::Length));
        }
        Some(Ok(()))
    }

    /// The status byte the device wrote for the answered request.
    pub fn status(&self) -> u8 {
        self.header.view.r8(STATUS_OFFSET)
    }
}
